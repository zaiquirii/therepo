use std::collections::VecDeque;

use bevy::prelude::*;

use super::packets::{spawn_packet, Packet, PacketDelivered, PacketDestroyed, ResourceType};

/// Possible resource types
/// * build
/// * repair
/// * energy

pub struct ResourceSupplier {
    pub inventory: u32,
}

pub struct ResourceConsumer {
    pub max: u32,
    pub current: u32,
    pub in_transit: u32,
    pub units_per_packet: u32,
    pub packet_remainder: u32,
}

impl ResourceConsumer {
    pub fn new(max: u32, charge_per_packet: u32) -> Self {
        Self {
            max,
            units_per_packet: charge_per_packet,
            current: 0,
            in_transit: 0,
            packet_remainder: 0,
        }
    }

    /// Count of resources that still need to be sent to this requester
    /// Hidden outside of module
    fn resources_to_send(&self) -> u32 {
        assert!(
            self.current + self.in_transit <= self.max,
            "Current and in_transit should never exceed max"
        );
        self.max - self.current - self.in_transit
    }

    /// Use some of the current resource
    /// Returns if enough resource was available
    /// Nothing is consumed if amount > current
    pub fn consume(&mut self, amount: u32) -> bool {
        let mut available = self.current * self.units_per_packet + self.packet_remainder;
        if available >= amount {
            available -= amount;
            self.current = available / self.units_per_packet;
            self.packet_remainder = available % self.units_per_packet;
            true
        } else {
            false
        }
    }
}

macro_rules! define_resource {
    ($supplier:ident, $requester:ident) => {
        #[derive(Component, Deref, DerefMut)]
        pub struct $supplier(ResourceSupplier);

        impl $supplier {
            pub fn new(inventory: u32) -> Self {
                Self(ResourceSupplier { inventory })
            }
        }

        #[derive(Component, Deref, DerefMut)]
        pub struct $requester(ResourceConsumer);

        impl $requester {
            pub fn new(max: u32, charge_per_packet: u32) -> Self {
                Self(ResourceConsumer::new(max, charge_per_packet))
            }
        }
    };
}

define_resource!(EnergySupplier, EnergyConsumer);
// define_resource!(BuildSupplier, BuildRequester);
// define_resource!(RepairSupplier, RepairRequester);

pub struct DispatchInfo {
    pub dispatch_timer: Timer,
}

impl Default for DispatchInfo {
    fn default() -> Self {
        Self {
            dispatch_timer: Timer::from_seconds(1.0, true),
        }
    }
}

macro_rules! dispatch_resource {
    ($commands:ident, $supplier:ident, $supplier_query:ident, $requester:ident, $requester_query:ident, $resource_type:expr) => {
        // Collect producers and receivers, filtering out those that don't need/have anything
        let mut producers: VecDeque<(Entity, Mut<$supplier>)> = VecDeque::new();
        for (entity, producer) in $supplier_query.iter_mut() {
            if producer.inventory > 0 {
                producers.push_back((entity, producer));
            }
        }

        let mut requesters: Vec<(Entity, Mut<$requester>)> = Vec::new();
        for (entity, requester) in $requester_query.iter_mut() {
            let request = requester.resources_to_send();
            if request > 0 {
                requesters.push((entity, requester));
            }
        }

        // Nothing to ship, exit early
        if producers.is_empty() {
            return;
        }
        let mut current_producer = producers.pop_front().unwrap();
        for mut requester in requesters {
            requester.1.in_transit += 1;
            current_producer.1.inventory -= 1;
            spawn_packet(
                &mut $commands,
                Packet::new($resource_type, current_producer.0, requester.0),
            );

            if current_producer.1.inventory == 0 {
                current_producer = match producers.pop_front() {
                    Some(producer) => producer,
                    // Once all current producers have been exhausted there is no
                    // reason to continue iterating through receivers
                    None => break,
                }
            }
        }
    };
}

/// Sends out packets to be delivered
pub fn dispatch_orders_system(
    mut commands: Commands,
    time: Res<Time>,
    mut dispatch_info: ResMut<DispatchInfo>,
    mut q_energy_supplier: Query<(Entity, &mut EnergySupplier)>,
    mut q_energy_requester: Query<(Entity, &mut EnergyConsumer)>,
) {
    /*
    Packets are sent to receivers in a round robin fashion
    */
    if !dispatch_info.dispatch_timer.tick(time.delta()).finished() {
        return;
    }

    dispatch_resource!(
        commands,
        EnergySupplier,
        q_energy_supplier,
        EnergyConsumer,
        q_energy_requester,
        ResourceType::Energy
    );
}

pub fn handle_packet_events(
    mut ev_delivered: EventReader<PacketDelivered>,
    mut ev_destroyed: EventReader<PacketDestroyed>,
    mut q_receivers: Query<&mut EnergyConsumer>,
) {
    for event in ev_delivered.iter() {
        if let Ok(mut receiver) = q_receivers.get_mut(event.packet.dst) {
            receiver.current += 1;
            receiver.in_transit -= 1;
        }
    }

    for event in ev_destroyed.iter() {
        // The receiver is probably missing as that is the main cause of packet loss
        // But checking here just in case.
        if let Ok(mut receiver) = q_receivers.get_mut(event.packet.dst) {
            receiver.in_transit -= 1;
        }
    }
}

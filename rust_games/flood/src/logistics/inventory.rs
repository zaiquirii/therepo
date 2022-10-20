use std::collections::VecDeque;

use bevy::prelude::*;

use super::{
    network::NodeId,
    packets::{spawn_packet, Packet, PacketDelivered, PacketDestroyed, ResourceType},
};

pub struct Resources {
    pub ammo: u32,
}

#[derive(Component)]
pub struct Producer {
    pub inventory: Resources,
}

#[derive(Component)]
pub struct Receiver {
    pub requests: Resources,
    pub in_transit: Resources,
}

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

/// Sends out packets to be delivered
pub fn dispatch_orders_system(
    mut commands: Commands,
    time: Res<Time>,
    mut dispatch_info: ResMut<DispatchInfo>,
    mut q_producers: Query<(Entity, &mut Producer)>,
    mut q_receivers: Query<(Entity, &mut Receiver)>,
) {
    /*
    Packets are sent to receivers in a round robin fashion
    */
    if !dispatch_info.dispatch_timer.tick(time.delta()).finished() {
        return;
    }

    // Collect producers and receivers, filtering out those that don't need/have anything
    let mut producers: VecDeque<(Entity, Mut<Producer>)> = VecDeque::new();
    for (entity, producer) in q_producers.iter_mut() {
        if producer.inventory.ammo > 0 {
            producers.push_back((entity, producer));
        }
    }

    let mut receivers: Vec<(Entity, Mut<Receiver>)> = Vec::new();
    for (entity, receiver) in q_receivers.iter_mut() {
        let ammo_request = receiver.requests.ammo;
        if ammo_request > 0 && receiver.in_transit.ammo < ammo_request {
            receivers.push((entity, receiver));
        }
    }

    // Nothing to ship, exit early
    if producers.is_empty() {
        return;
    }
    let mut current_producer = producers.pop_front().unwrap();
    for mut receiver in receivers {
        receiver.1.in_transit.ammo += 1;
        current_producer.1.inventory.ammo -= 1;
        spawn_packet(
            &mut commands,
            Packet::new(ResourceType::Ammo, current_producer.0, receiver.0),
        );

        if current_producer.1.inventory.ammo == 0 {
            current_producer = match producers.pop_front() {
                Some(producer) => producer,
                // Once all current producers have been exhausted there is no
                // reason to continue iterating through receivers
                None => break,
            }
        }
    }
}

pub fn handle_packet_events(
    mut ev_delivered: EventReader<PacketDelivered>,
    mut ev_destroyed: EventReader<PacketDestroyed>,
    mut q_receivers: Query<&mut Receiver>,
) {
    for event in ev_delivered.iter() {
        if let Ok(mut receiver) = q_receivers.get_mut(event.packet.dst) {
            receiver.requests.ammo -= 1;
            receiver.in_transit.ammo -= 1;
        }
    }

    for event in ev_destroyed.iter() {
        // The receiver is probably missing as that is the main cause of packet loss
        // But checking here just in case.
        if let Ok(mut receiver) = q_receivers.get_mut(event.packet.dst) {
            receiver.in_transit.ammo -= 1;
        }
    }
}

use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_prototype_lyon::prelude::*;

use crate::z_levels;

use super::network::Network;

pub struct PacketDelivered {
    pub entity: Entity,
    pub packet: Packet,
}

pub struct PacketDestroyed {
    pub entity: Entity,
    pub packet: Packet,
}

#[derive(Copy, Clone)]
pub enum ResourceType {
    // Build,
    // Repair,
    Energy,
}

#[derive(Component, Copy, Clone)]
pub struct Packet {
    pub resource_type: ResourceType,
    pub src: Entity,
    // Network NodeId this packet is trying to reach
    pub dst: Entity,
}

impl Packet {
    pub fn new(resource_type: ResourceType, src: Entity, dst: Entity) -> Self {
        Packet {
            resource_type,
            src,
            dst,
        }
    }
}

pub fn spawn_packet(commands: &mut Commands, packet: Packet) {
    let shape = shapes::Rectangle {
        extents: Vec2::new(1.0, 1.0),
        origin: RectangleOrigin::BottomLeft,
    };

    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shape,
            DrawMode::Fill(FillMode::color(Color::RED)),
            Transform::from_xyz(0.0, 0.0, z_levels::INFRASTRUCTURE),
        ))
        .insert(packet);
}

const PACKET_SPEED: f32 = 5.0;
const PACKET_DELIVERED_DISTANCE: f32 = 1.0;

pub fn move_packet_system(
    mut commands: Commands,
    time: Res<Time>,
    log_net: Res<Network>,
    mut q_packets: Query<(Entity, &mut Transform, &mut Packet)>,
    mut ev_destroyed: EventWriter<PacketDestroyed>,
    mut ev_delivered: EventWriter<PacketDelivered>,
) {
    for (entity, mut transform, packet) in q_packets.iter_mut() {
        if packet.is_added() {
            if let Some(src_node) = log_net.nodes().get(&packet.src.id()) {
                transform.translation = src_node.location.extend(z_levels::PACKET);
            } else {
                eprintln!("src_node missing {}", packet.src.id());
                commands.entity(entity).despawn();
                ev_destroyed.send(PacketDestroyed {
                    entity,
                    packet: *packet,
                });
            }
        } else {
            if let Some(dst_node) = log_net.nodes().get(&packet.dst.id()) {
                let delta = dst_node.location - transform.translation.xy();
                let distance = delta.length();
                if distance <= PACKET_DELIVERED_DISTANCE {
                    ev_delivered.send(PacketDelivered {
                        entity,
                        packet: *packet,
                    });
                    commands.entity(entity).despawn();
                } else {
                    let update = delta.normalize_or_zero() * PACKET_SPEED * time.delta_seconds();
                    transform.translation += update.extend(0.0);
                }
            } else {
                eprintln!("dst_node missing {}", packet.dst.id());
                ev_destroyed.send(PacketDestroyed {
                    entity,
                    packet: *packet,
                });
                commands.entity(entity).despawn();
            }
        }
    }
}

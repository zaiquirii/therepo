use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_prototype_debug_lines::DebugLines;

use super::{
    inventory::{dispatch_orders_system, handle_packet_events, DispatchInfo},
    network::{Network, NodeId, NodeType},
    packets::{move_packet_system, PacketDelivered, PacketDestroyed},
};

pub struct LogisticsNodeRemoved(pub Entity);

#[derive(Component)]
pub struct LogisticsNode {
    pub range: f32,
}

impl LogisticsNode {
    pub fn new(range: f32) -> Self {
        LogisticsNode { range }
    }
}

pub fn setup_logistics_system(mut commands: Commands) {
    commands.insert_resource(Network::default());
}

/// This system needs to run every frame to ensure things work with the ui
pub fn sync_new_logistics_nodes_system(
    mut log_net: ResMut<Network>,
    mut ev_log_node_removed: EventReader<LogisticsNodeRemoved>,
    q_new_nodes: Query<(Entity, &LogisticsNode, &Transform), Added<LogisticsNode>>,
) {
    for ev in ev_log_node_removed.iter() {
        let node = ev.0.id();
        log_net.remove_node(node);
    }

    for (entity, node, transform) in q_new_nodes.iter() {
        log_net.create_node(
            entity.id(),
            NodeType::Core,
            transform.translation.xy() + Vec2::splat(1.0),
            node.range,
        );
        let added_connections = log_net.reconcile_connections(entity.id());
        // TODO: Remove
        eprintln!(
            "Added connections for entity {}, {} new connections",
            entity.id(),
            added_connections
        );
    }
}

pub fn draw_connections_system(log_net: Res<Network>, mut lines: ResMut<DebugLines>) {
    let nodes = log_net.nodes();
    for node_id in nodes.keys() {
        let src_node = nodes.get(node_id).unwrap();
        if let Some(connections) = log_net.connections(*node_id) {
            for dst_node_id in connections {
                if let Some(dst_node) = nodes.get(dst_node_id) {
                    lines.line_colored(
                        src_node.location.extend(5.0),
                        dst_node.location.extend(5.0),
                        0.0,
                        Color::WHITE,
                    );
                }
            }
        }
    }
}

pub struct LogisticsPlugin;

impl Plugin for LogisticsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LogisticsNodeRemoved>()
            .add_event::<PacketDelivered>()
            .add_event::<PacketDestroyed>()
            .init_resource::<DispatchInfo>()
            .add_system(dispatch_orders_system)
            .add_system(move_packet_system)
            .add_system(handle_packet_events);
    }
}

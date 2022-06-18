use bevy::{math::Vec3, prelude::Color};

use crate::{
    common::components::{pos::PosComponent, static_mesh::vertex::Vertex},
    plugins::chunk::resources::chunk::Chunk,
};

use self::triangulation_table::{get_index_by_voxels, TABLE};

use super::Voxel;

mod triangulation_table;

#[derive(Clone, Copy)]
struct VertexNode {
    index: usize,
    pos: Vec3,
}

fn get_node_dn() -> VertexNode {
    VertexNode {
        index: 0,
        pos: Vec3::new(0.5, 0.0, 1.0),
    }
}
fn get_node_de() -> VertexNode {
    VertexNode {
        index: 1,
        pos: Vec3::new(1.0, 0.0, 0.5),
    }
}
fn get_node_ds() -> VertexNode {
    VertexNode {
        index: 2,
        pos: Vec3::new(0.5, 0.0, 0.0),
    }
}
fn get_node_dw() -> VertexNode {
    VertexNode {
        index: 3,
        pos: Vec3::new(0.0, 0.0, 0.5),
    }
}

fn get_node_un() -> VertexNode {
    VertexNode {
        index: 4,
        pos: Vec3::new(0.5, 1.0, 1.0),
    }
}
fn get_node_ue() -> VertexNode {
    VertexNode {
        index: 5,
        pos: Vec3::new(1.0, 1.0, 0.5),
    }
}
fn get_node_us() -> VertexNode {
    VertexNode {
        index: 6,
        pos: Vec3::new(0.5, 1.0, 0.0),
    }
}
fn get_node_uw() -> VertexNode {
    VertexNode {
        index: 7,
        pos: Vec3::new(0.0, 1.0, 0.5),
    }
}

fn get_node_nw() -> VertexNode {
    VertexNode {
        index: 8,
        pos: Vec3::new(0.0, 0.5, 1.0),
    }
}
fn get_node_ne() -> VertexNode {
    VertexNode {
        index: 9,
        pos: Vec3::new(1.0, 0.5, 1.0),
    }
}
fn get_node_se() -> VertexNode {
    VertexNode {
        index: 10,
        pos: Vec3::new(1.0, 0.5, 0.0),
    }
}
fn get_node_sw() -> VertexNode {
    VertexNode {
        index: 11,
        pos: Vec3::new(0.0, 0.5, 0.0),
    }
}

const NODES_POS_COUNT: usize = 12;
fn get_base_nodes() -> [VertexNode; NODES_POS_COUNT] {
    [
        get_node_dn(),
        get_node_de(),
        get_node_ds(),
        get_node_dw(),
        get_node_un(),
        get_node_ue(),
        get_node_us(),
        get_node_uw(),
        get_node_nw(),
        get_node_ne(),
        get_node_se(),
        get_node_sw(),
    ]
}

type Nodes = [Voxel; NODES_POS_COUNT];
type VoxelsBlock = [[[Voxel; 2]; 2]; 2];

fn get_voxel(chunk: &Chunk, pos: PosComponent) -> Voxel {
    match chunk.get_voxel(pos) {
        Some(voxel) => voxel,
        _ => Voxel {
            value: 0.,
            color: Color::BLACK,
        },
    }
}

fn get_voxels_for_vertex(chunk: &Chunk, base_pos: PosComponent) -> VoxelsBlock {
    let voxels: [[[Voxel; 2]; 2]; 2] = [
        [
            [
                get_voxel(chunk, base_pos + PosComponent::new(0, 0, 0)),
                get_voxel(chunk, base_pos + PosComponent::new(0, 0, 1)),
            ],
            [
                get_voxel(chunk, base_pos + PosComponent::new(0, 1, 0)),
                get_voxel(chunk, base_pos + PosComponent::new(0, 1, 1)),
            ],
        ],
        [
            [
                get_voxel(chunk, base_pos + PosComponent::new(1, 0, 0)),
                get_voxel(chunk, base_pos + PosComponent::new(1, 0, 1)),
            ],
            [
                get_voxel(chunk, base_pos + PosComponent::new(1, 1, 0)),
                get_voxel(chunk, base_pos + PosComponent::new(1, 1, 1)),
            ],
        ],
    ];
    return voxels;
}

fn chose_voxel_for_node(a: Voxel, b: Voxel) -> Voxel {
    if a.value < 0. {
        return Voxel {
            color: b.color,
            value: (-a.value) / (b.value - a.value),
        };
    }
    if b.value < 0. {
        return Voxel {
            color: a.color,
            value: 1.0 - (-b.value) / (a.value - b.value),
        };
    }
    return Voxel {
        value: 0.,
        color: Color::BLACK,
    };
}

fn get_vertex_nodes(voxels: VoxelsBlock) -> Nodes {
    let mut result: Nodes = [Voxel {
        value: 0.,
        color: Color::BLACK,
    }; NODES_POS_COUNT];

    result[get_node_ds().index] = chose_voxel_for_node(voxels[0][0][0], voxels[1][0][0]);
    result[get_node_de().index] = chose_voxel_for_node(voxels[1][0][0], voxels[1][0][1]);
    result[get_node_dn().index] = chose_voxel_for_node(voxels[0][0][1], voxels[1][0][1]);
    result[get_node_dw().index] = chose_voxel_for_node(voxels[0][0][0], voxels[0][0][1]);

    result[get_node_ne().index] = chose_voxel_for_node(voxels[1][0][1], voxels[1][1][1]);
    result[get_node_nw().index] = chose_voxel_for_node(voxels[0][0][1], voxels[0][1][1]);
    result[get_node_se().index] = chose_voxel_for_node(voxels[1][0][0], voxels[1][1][0]);
    result[get_node_sw().index] = chose_voxel_for_node(voxels[0][0][0], voxels[0][1][0]);

    result[get_node_us().index] = chose_voxel_for_node(voxels[0][1][0], voxels[1][1][0]);
    result[get_node_ue().index] = chose_voxel_for_node(voxels[1][1][0], voxels[1][1][1]);
    result[get_node_un().index] = chose_voxel_for_node(voxels[0][1][1], voxels[1][1][1]);
    result[get_node_uw().index] = chose_voxel_for_node(voxels[0][1][0], voxels[0][1][1]);

    return result;
}

fn shift_node_pos(pos: Vec3, value: f32) -> Vec3 {
    if pos.x == 0.5 {
        return Vec3::new(value, pos.y, pos.z);
    }
    if pos.y == 0.5 {
        return Vec3::new(pos.x, value, pos.z);
    }
    if pos.z == 0.5 {
        return Vec3::new(pos.x, pos.y, value);
    }

    panic!("failed to process pos {:?}", pos);
}

fn append_triangle(
    pos: PosComponent,
    vertex: &mut Vec<Vertex>,
    nodes: Nodes,
    a: VertexNode,
    b: VertexNode,
    c: VertexNode,
) {
    let a_v = nodes[a.index];
    let b_v = nodes[b.index];
    let c_v = nodes[c.index];

    if a_v.value < 0. || a_v.value < 0. || c_v.value < 0. {
        return;
    }

    let pos_vec = Vec3::new(pos.x as f32, pos.y as f32, pos.z as f32);

    let a_pos = shift_node_pos(a.pos, a_v.value) + pos_vec;
    let b_pos = shift_node_pos(b.pos, b_v.value) + pos_vec;
    let c_pos = shift_node_pos(c.pos, c_v.value) + pos_vec;

    let normal = (c_pos - a_pos).cross(b_pos - a_pos).normalize();

    vertex.push(Vertex {
        color: a_v.color.clone(),
        normal,
        pos: c_pos,
    });
    vertex.push(Vertex {
        color: a_v.color.clone(),
        normal,
        pos: b_pos,
    });
    vertex.push(Vertex {
        color: a_v.color.clone(),
        normal,
        pos: a_pos,
    });
}

pub fn append_vertex(pos: PosComponent, chunk: &Chunk, vertices: &mut Vec<Vertex>) {
    let voxels = get_voxels_for_vertex(chunk, pos);
    let nodes = get_vertex_nodes(voxels);

    let triangle_points = TABLE[get_index_by_voxels(voxels)];

    let mut triangle_offset = 0;

    let nodes_arr = get_base_nodes();

    while triangle_points[triangle_offset] != -1 {
        let a = nodes_arr[triangle_points[triangle_offset] as usize];
        let b = nodes_arr[triangle_points[triangle_offset + 1] as usize];
        let c = nodes_arr[triangle_points[triangle_offset + 2] as usize];

        append_triangle(pos, vertices, nodes, a, b, c);

        triangle_offset += 3;
    }
}

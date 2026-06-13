/*  LangViewer - A tool for visualising languages, grammars, and automata.
    Copyright (C) 2026 Campbell Rowland>

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU Affero General Public License as
    published by the Free Software Foundation, either version 3 of the
    License, or (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU Affero General Public License for more details.

    You should have received a copy of the GNU Affero General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>. */

use iced::{Vector, widget::canvas::{self, path::Builder}};
use rstar::{AABB, RTree};

use crate::graphics::dfa_mode::NODE_SIZE;

use super::dfa_mode::Node;

/// How far to offset parallel connections between the same two nodes
const PARALLEL_OFFSET: f32 = 50.0;

const NUDGE_THRESHOLD: f32 = (NODE_SIZE << 3) as f32; // distance at which to nudge the curve away from a nearby node
/// The angle of the arrowhead in radians, relative to the negative tangent vector of the end of the connection.
const ARROW_ANGLE: f32 = std::f32::consts::PI / 6.0;
/// The scale factor for the size of the arrowhead relative to the node size.
const ARROW_SCALE: f32 = 0.3;

/// Computes the vector for the left side of the arrowhead based on the offset vector.
macro_rules! left {
    ($off:expr) => {
        iced::Vector::new(-$off.x*f32::cos(ARROW_ANGLE)+$off.y*f32::sin(ARROW_ANGLE),
        -$off.x*f32::sin(ARROW_ANGLE)-$off.y*f32::cos(ARROW_ANGLE))
    };
}

/// Computes the vector for the right side of the arrowhead based on the offset vector.
macro_rules! right {
    ($off:expr) => {
        iced::Vector::new(-$off.x*f32::cos(-ARROW_ANGLE)+$off.y*f32::sin(-ARROW_ANGLE),
        -$off.x*f32::sin(-ARROW_ANGLE)-$off.y*f32::cos(-ARROW_ANGLE))
    };
}

/// Represents a connection between two nodes in the DFA with a symbol for transition.
#[derive(Debug, Clone, Copy)]
pub struct Connection {
    /// The starting point and index of the node
    pub start: (iced::Point<f32>, usize),
    /// The ending point and index of the node
    pub end: (iced::Point<f32>, usize), 
    /// The symbol associated with the transition
    pub symbol: char, 
}

/// Creates a Path for the arrow representing the connection at `conn_idx`
pub fn compute_arrow(conn_idx: usize,
    parallel: &[usize], nodes: &RTree<Node>, conns: &[Connection]) -> canvas::Path
{
    let conn = conns[conn_idx];
    let midpoint: iced::Point<f32> = iced::Point::new((conn.start.0.x + conn.end.0.x) / 2.0,
        (conn.start.0.y + conn.end.0.y) / 2.0);
    let edge_vec = conn.end.0 - conn.start.0;
    let norm = (edge_vec.x.powi(2) + edge_vec.y.powi(2)).sqrt();
    let perp = iced::Vector::new(-edge_vec.y / norm, edge_vec.x / norm);

    let ind = parallel.iter().position(|&idx| idx == conn_idx).unwrap();
    let n = parallel.len();

    let edge_rank = ind as f32 - (n as f32 - 1.0) / 2.0;
    let offset = edge_rank * PARALLEL_OFFSET;

    let mut cp = midpoint + perp * offset;

    let bbox = AABB::from_center(
        Node { pos: midpoint, .. },
        norm + (NODE_SIZE << 2) as f32);
    for nearby_node in nodes.locate_in_envelope(bbox) {
        let to_node = nearby_node.pos - cp;
        let dist = (to_node.x.powi(2) + to_node.y.powi(2)).sqrt();
        if dist < NUDGE_THRESHOLD && nearby_node.index.unwrap() != conn.start.1 && nearby_node.index.unwrap() != conn.end.1 {
            let size = (to_node.x.powi(2) + to_node.y.powi(2)).sqrt();
            let normal = Vector::new(to_node.x / size, to_node.y / size);
            cp -= normal * (NUDGE_THRESHOLD - dist) * 0.5;
        }
    }
    let start_centre = conn.start.0;
    let end_centre = conn.end.0;
    let start_tangent = cp - start_centre;
    let st_len = (start_tangent.x.powi(2) + start_tangent.y.powi(2)).sqrt();
    let start_dir = Vector::new(start_tangent.x / st_len, start_tangent.y / st_len);

    let end_tangent = end_centre - cp;
    let et_len = (end_tangent.x.powi(2) + end_tangent.y.powi(2)).sqrt();
    let end_dir = Vector::new(end_tangent.x / et_len, end_tangent.y / et_len);

    // Where the curve leaves the start circle and enters the end circle
    let curve_start = start_centre + start_dir * NODE_SIZE as f32;
    let curve_end   = end_centre   - end_dir   * NODE_SIZE as f32;

    let arrow_off = end_dir * (NODE_SIZE as f32 * ARROW_SCALE); // scale as you like
        
    let mut build = Builder::new();
    build.move_to(curve_start);
    build.quadratic_curve_to(cp, curve_end);

    build.move_to(curve_end);
    build.line_to(curve_end + left!(arrow_off));
    build.move_to(curve_end);
    build.line_to(curve_end + right!(arrow_off));
    build.build()
}

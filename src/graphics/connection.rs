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

const PARALLEL_OFFSET: f32 = 20.0;
const NUDGE_THRESHOLD: f32 = NODE_SIZE as f32 * 2.0;
const ARROW_ANGLE: f32 = std::f32::consts::PI / 6.0;

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

    let edge_rank = parallel.iter().position(|&idx| idx == conn_idx).unwrap() as f32
        - (parallel.len() as f32 - 1.0) / 2.0;
    let n = parallel.len() as f32;
    let offset = (edge_rank as f32 - (n - 1.0) / 2.0) * PARALLEL_OFFSET;

    let mut cp = midpoint + perp * offset;

    let bbox = AABB::from_corners(
        Node { pos: conn.start.0, .. },
        Node { pos: conn.end.0, ..});
    for nearby_node in nodes.locate_in_envelope(bbox) {
        let to_node = nearby_node.pos - cp;
        let dist = to_node.x.powi(2) + to_node.y.powi(2);
        if dist < NUDGE_THRESHOLD {
            let size = (to_node.x.powi(2) + to_node.y.powi(2)).sqrt();
            let norm = Vector::new(to_node.x / size, to_node.y / size);
            cp -= norm * (NUDGE_THRESHOLD - dist) * 0.5;
        }
    }
    let mut build: Builder = Builder::new();

    let dist = ((conn.end.0.x - conn.start.0.x).powi(2) + (conn.end.0.y - conn.start.0.y).powi(2)).sqrt();
    let off = iced::Vector::new(((conn.end.0.x - conn.start.0.x) / dist) * NODE_SIZE as f32,
        ((conn.end.0.y - conn.start.0.y) / dist )* NODE_SIZE as f32);
    let fin = conn.end.0 - off;
        
    build.move_to(conn.start.0 + off);
    build.quadratic_curve_to(cp, fin);

    build.move_to(fin);
    build.line_to(fin + left!(off));
    build.move_to(fin);
    build.line_to(fin + right!(off));
    build.build()
}

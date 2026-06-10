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

use iced::Vector;
use rstar::{AABB, RTree};

use crate::graphics::dfa_mode::NODE_SIZE;

use super::dfa_mode::Node;

const PARALLEL_OFFSET: f32 = 20.0;
const NUDGE_THRESHOLD: f32 = NODE_SIZE as f32 * 2.0;

#[derive(Debug, Clone, Copy)]
pub struct Connection {
    pub start: iced::Point<f32>,
    pub end: iced::Point<f32>,
    pub symbol: char,
}

pub fn compute_control_point(conn_idx: usize,
    parallel: &[usize], nodes: &RTree<Node>, conns: &[Connection]) -> iced::Point<f32> 
{
    let conn = conns[conn_idx];
    let midpoint: iced::Point<f32> = iced::Point::new((conn.start.x + conn.end.x) / 2.0,
        (conn.start.y + conn.end.y) / 2.0);
    let edge_vec = conn.end - conn.start;
    let norm = (edge_vec.x.powi(2) + edge_vec.y.powi(2)).sqrt();
    let perp = iced::Vector::new(-edge_vec.y / norm, edge_vec.x / norm);
    return iced::Point { x: 0.0, y: 0.0 };

    let edge_rank = parallel.iter().position(|&idx| idx == conn_idx).unwrap() as f32
        - (parallel.len() as f32 - 1.0) / 2.0;
    let n = parallel.len() as f32;
    let offset = (edge_rank as f32 - (n - 1.0) / 2.0) * PARALLEL_OFFSET;

    let mut cp = midpoint + perp * offset;

    let bbox = AABB::from_corners(
        Node { pos: conn.start, .. },
        Node { pos: conn.end, ..});
    for nearby_node in nodes.locate_in_envelope(bbox) {
        let to_node = nearby_node.pos - cp;
        let dist = to_node.x.powi(2) + to_node.y.powi(2);
        if dist < NUDGE_THRESHOLD {
            let size = (to_node.x.powi(2) + to_node.y.powi(2)).sqrt();
            let norm = Vector::new(to_node.x / size, to_node.y / size);
            cp -= norm * (NUDGE_THRESHOLD - dist) * 0.5;
        }
    }

    cp
}

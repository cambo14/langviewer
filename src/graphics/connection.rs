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

use std::hash::{Hash, Hasher};
use iced::{Color, Vector, widget::{canvas::{self, path::Builder}, text::LineHeight}};
use rstar::{AABB, RTree};

use crate::graphics::{dfa_mode::NODE_SIZE};

use super::dfa_mode::Node;

/// The font size for the transition symbol text on the connection arrows.
const SYMBOL_SIZE: iced::Pixels = iced::Pixels(16.0);

/// How far to offset parallel connections between the same two nodes
const PARALLEL_OFFSET: f32 = 50.0;

/// distance at which to nudge the curve away from a nearby node
const NUDGE_THRESHOLD: f32 = (NODE_SIZE << 3) as f32;
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
#[derive(Debug, Clone)]
pub struct Connection {
    /// The starting point and index of the node
    pub start: (iced::Point<f32>, usize),
    /// The ending point and index of the node
    pub end: (iced::Point<f32>, usize), 
    /// The symbol associated with the transition
    pub symbol: char, 
    /// The label location
    pub label_loc: iced::Point<f32>,
    /// The index of the connection in the RTree, used for efficient updates when moving nodes
    pub index: Option<usize> = None,
    /// The path object representing the curve of the connection, used for rendering on the canvas
    pub path: Option<canvas::Path> = None,
}


impl Connection {
    /// Creates a new connection with the given parameters
    /// 
    /// # Arguments
    /// * `start` - The starting point and index of the node
    /// * `end` - The ending point and index of the node
    /// * `symbol` - The symbol associated with the transition
    /// * `label_loc` - The location of the label for the connection
    /// * `index` - The index of the connection in the RTree
    /// * `path` - The path object representing the curve of the connection, used for rendering on the canvas
    /// 
    /// # Returns
    /// A new connection with the given parameters
    pub fn new(
        start: (iced::Point<f32>, usize),
        end: (iced::Point<f32>, usize),
        symbol: char,
        label_loc: iced::Point<f32>,
        index: Option<usize>,
        path: Option<canvas::Path>,
    ) -> Self {
        Connection {
            start,
            end,
            symbol,
            label_loc,
            index,
            path,
        }
    }
}

impl rstar::Point for Connection {
    type Scalar = f32;

    fn generate(mut generator: impl FnMut(usize) -> Self::Scalar) -> Self {
        Connection {
            start: (iced::Point::new(0.0, 0.0), 0),
            end: (iced::Point::new(0.0, 0.0), 0),
            symbol: '\0',
            label_loc: iced::Point::new(generator(0), generator(1)),
            ..
        }
    }
    const DIMENSIONS: usize = 2;

    fn nth(&self, index: usize) -> Self::Scalar {
        match index {
            0 => self.label_loc.x,
            1 => self.label_loc.y,
            _ => unreachable!()
        }
    }

    fn nth_mut(&mut self, index: usize) -> &mut Self::Scalar {
        match index {
            0 => &mut self.label_loc.x,
            1 => &mut self.label_loc.y,
            _ => unreachable!()
        }
    }
}

impl PartialEq for Connection {
    fn eq(&self, other: &Self) -> bool {
        self.start.1 == other.start.1 && self.end.1 == other.end.1 && self.symbol == other.symbol
    }
}
impl Eq for Connection {}

impl Hash for Connection {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.start.1.hash(state);
        self.end.1.hash(state);
        self.symbol.hash(state);
    }
}

/// Generates the graphical component for an arrow representing the connection at `conn_idx`
/// 
/// # Arguments
/// * `conn_idx` - The index of the connection for which to compute the arrow
/// * `parallel` - A slice of indices of connections that are parallel to the connection at `conn_idx` and their direction 
///   (true for same direction, false for opposite), used to determine how much to offset the curve for this connection from the midpoint between the start and end nodes
/// * `nodes` - The RTree containing the nodes of the DFA, used for spatial queries to nudge the curve away from nearby nodes
/// * `conns` - A HashMap of all connections in the DFA, used to retrieve the details of the connection at `conn_idx` and its parallel connections
/// 
/// # Returns
/// A tuple containing the graphical objects for the transition symbol and the path of the arrow to be drawn on the canvas
pub fn compute_arrow(conn: &Connection,
    parallel: &Vec<&Connection>, nodes: &RTree<Node>, conns: &RTree<Connection>) -> (iced::Point, canvas::Path)
{
    let conn = conns.nearest_neighbor(rstar::Point::generate(
        |i| if i == 0 { conn.label_loc.x } else if i == 1 { conn.label_loc.y } else { 0.0 }
    )).unwrap();
    let midpoint: iced::Point<f32> = iced::Point::new((conn.start.0.x + conn.end.0.x) / 2.0,
        (conn.start.0.y + conn.end.0.y) / 2.0);
    let edge_vec = conn.end.0 - conn.start.0;
    let norm = (edge_vec.x.powi(2) + edge_vec.y.powi(2)).sqrt();
    let perp = iced::Vector::new(-edge_vec.y / norm, edge_vec.x / norm);

    let ind = parallel.iter().position(|&idx| idx.index.unwrap() == conn.index.unwrap_or(0)).unwrap();
    let n = parallel.len();

    let edge_rank = ind as f32 - (n as f32 - 1.0) / 2.0;
    let offset = edge_rank * PARALLEL_OFFSET;

    let mut cp = if conn.start.0.x - conn.end.0.x > 0.0 {
        midpoint + perp * offset
    } else {
        midpoint - perp * offset
    };

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

    let arrow_off = end_dir * (NODE_SIZE as f32 * ARROW_SCALE);
        
    let mut build = Builder::new();
    build.move_to(curve_start);
    build.quadratic_curve_to(cp, curve_end);

    build.move_to(curve_end);
    build.line_to(curve_end + left!(arrow_off));
    build.move_to(curve_end);
    build.line_to(curve_end + right!(arrow_off));

    let curve_point = iced::Point::new(
        0.25 * curve_start.x + 0.5 * cp.x + 0.25 * curve_end.x,
        0.25 * curve_start.y + 0.5 * cp.y + 0.25 * curve_end.y) + 
        if conn.start.0.x - conn.end.0.x > 0.0 { perp * 10.0} else { -perp * 10.0 };
    (
        curve_point ,
        build.build()
    )
}

/// Generates the graphical text representing the connection, displaying
/// the transition symbol at the label location of the connection
pub fn connection_text(conn: &Connection) -> canvas::Text {
    canvas::Text { content: conn.symbol.to_string(), position: conn.label_loc,
        color: Color::BLACK, size: SYMBOL_SIZE, font: iced::Font::DEFAULT,
        align_x: iced::widget::text::Alignment::Center, align_y: iced::alignment::Vertical::Center,
        line_height: LineHeight::Relative(1.0), max_width: 5.0,
        shaping: iced::widget::text::Shaping::Auto,}
}

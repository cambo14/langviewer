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

//! The main entry point for the application, creating a graphical instance and running it


#![feature(default_field_values)]

/// Handles the graphical implementation of the program
mod graphics;

/// Main entry point of the program. Initializes the graphics module and starts the application.
fn main() -> iced::Result{
    env_logger::init();
    graphics::initialise()
}

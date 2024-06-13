/*
 * MIT License
 * 
 * Copyright (c) 2024 Leon Cotten
 * 
 * This language is provided under the MIT Licence.
 * See LICENSE for more information.
 */

extern crate clap;
use clap::{
    App,
    Arg,
    SubCommand
};

pub fn get_synapse_subcommand() -> App<'static> {
    SubCommand::with_name("synapse")
        .about("the synapse dependency manager subcommand")
        .subcommand(
            SubCommand::with_name("new")
                .about("creates a new brainfpp project")
                .arg(
                    Arg::new("name")
                        .value_name("NAME")
                        .help("the name of the project")
                        .required(true)
                        .index(1)
                )
        )
        .subcommand(
            SubCommand::with_name("add")
                .about("adds a dependency to the synapse.toml file")
                .arg(
                    Arg::new("neuron")
                        .value_name("NEURON")
                        .help("the filepath to the dependency")
                        .index(1)
                )
        )
        .subcommand(
            SubCommand::with_name("remove")
                .about("removes a dependency from the synapse.toml file")
                .arg(
                    Arg::new("neuron")
                        .value_name("NEURON")
                        .help("the filepath to the dependency")
                        .index(1)
                )
        )
}
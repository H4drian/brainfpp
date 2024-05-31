/*
 * MIT License
 * 
 * Copyright (c) 2024 Leon Cotten
 * 
 * This language is provided under the MIT Licence.
 * See LICENSE for more information.
 */ 

/// The universal trait implemented for every struct in the bfpp api.
pub trait BaseTrait {
    fn new() -> Self;
    fn reset(&mut self) -> ();
}
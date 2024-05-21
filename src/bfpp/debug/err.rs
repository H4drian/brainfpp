/*
 * MIT License
 * 
 * Copyright (c) 2024 Leon Cotten
 * 
 * This language is provided under the MIT Licence.
 * See LICENSE for more information.
 */

pub struct BfppError {
    pub message: String,
    pub errcode: u8,
    pub file: String,
    pub line: usize
}

pub struct BfppWarning {
    pub message: String,
    pub file: String,
    pub line: usize
}
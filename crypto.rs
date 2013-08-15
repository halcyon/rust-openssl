/*
 * Copyright 2011 Google Inc.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#[link(name = "crypto",
       vers = "0.2",
       uuid = "38297409-b4c2-4499-8131-a99a7e44dad3")];
#[crate_type = "lib"];

pub mod hash;
pub mod pkey;
pub mod symm;
pub mod pkcs5;
pub mod rand;
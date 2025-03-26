use rustengan::*;

use anyhow::Context;
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    io::{StdoutLock, Write},
    time::Duration,
};
pub mod aggregate;
pub mod borrowck_errors;
pub mod def_use;
pub mod elaborate_drops;
pub mod patch;

mod alignment;
pub mod collect_writes;
mod graphviz;
pub mod liveness;
pub(crate) mod pretty;

pub use self::aggregate::expand_aggregate;
pub use self::alignment::is_disaligned;
pub use self::graphviz::write_node_label as write_graphviz_node_label;
pub use self::graphviz::{graphviz_safe_def_name, write_mir_graphviz};
pub use self::pretty::{dump_enabled, dump_mir, write_mir_pretty, PassWhere};

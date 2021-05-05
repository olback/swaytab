use {
    config::STConfig,
    error::{STError, STResult},
    std::{
        collections::HashMap,
        io::Write,
        process::{Command, Stdio},
    },
    swayipc::{
        reply::{Node, NodeType},
        Connection,
    },
};

mod config;
mod error;

fn collect_con_nodes(collection: &mut Vec<Node>, root: Node) {
    if root.node_type == NodeType::Con {
        collection.push(root);
    } else {
        for child in root.nodes {
            collect_con_nodes(collection, child);
        }
    }
}

fn main() -> STResult<()> {
    // Load config
    let conf = STConfig::load()?;

    // Connect to swayipc
    let mut con = Connection::new()?;

    // Get tree
    let tree = con.get_tree()?;

    // Get nodes
    let mut nodes = Vec::<Node>::new();
    collect_con_nodes(&mut nodes, tree);
    let map = nodes
        .into_iter()
        .filter_map(|n| match (n.name, n.pid) {
            (Some(name), Some(pid)) => Some((name, pid)),
            _ => None,
        })
        .collect::<HashMap<String, i32>>();

    // Get keys (window titles)
    let keys = map
        .iter()
        .map(|(key, _)| key.clone())
        .collect::<Vec<String>>()
        .join("\n");

    // Run menu
    let cmd_parts = conf.command.split(' ').collect::<Vec<&str>>();
    if cmd_parts.len() == 0 {
        return Err("Failed to parse command".to_string().into());
    }
    let mut menu_child = Command::new(cmd_parts[0])
        .args(&cmd_parts[1..])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    menu_child
        .stdin
        .as_mut()
        .ok_or(STError::Other("stdin is null".into()))?
        .write_all(keys.as_bytes())?;

    let menu_output = menu_child.wait_with_output()?;

    if !menu_output.status.success() {
        // User closed menu
        return Ok(());
    }

    let selected_item = String::from_utf8(menu_output.stdout)?.trim().to_string();
    let pid = map
        .get(&selected_item)
        .ok_or(STError::Other("Internal error".into()))?;

    // Send focus to selected application
    con.run_command(format!("[pid={}] focus", pid))?;

    Ok(())
}

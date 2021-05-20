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
    if root.node_type == NodeType::Con || root.node_type == NodeType::FloatingCon {
        collection.push(root);
    } else {
        for child in root.nodes {
            collect_con_nodes(collection, child);
        }
        for child in root.floating_nodes {
            collect_con_nodes(collection, child);
        }
    }
}

fn main() -> STResult<()> {
    // Handle args
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() == 2 {
        match args[1].as_str() {
            "--version" | "-v" => println!("Version: {}", env!("CARGO_PKG_VERSION")),
            "--gen-config" | "-g" => {
                STConfig::write_default()?;
                println!("Wrote default config to {:?}", STConfig::path()?);
            }
            _ => {}
        }
        return Ok(());
    }

    // Load config
    let conf = STConfig::load()?;

    // Connect to swayipc
    let mut con = Connection::new()?;

    // Get tree
    let tree = con.get_tree()?;

    // Get nodes
    let mut nodes = Vec::<Node>::new();
    collect_con_nodes(&mut nodes, tree);

    // If no windows are open, exit
    if nodes.len() == 0 {
        return Ok(());
    }

    let map = nodes
        .into_iter()
        .filter_map(|n| match (n.name, n.id) {
            (Some(name), id) => Some((name, id)),
            _ => None,
        })
        .collect::<HashMap<String, i64>>();

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
    let id = map
        .get(&selected_item)
        .ok_or(STError::Other("Internal error".into()))?;

    // Send focus to selected application
    con.run_command(format!("[con_id={}] focus", id))?;

    Ok(())
}

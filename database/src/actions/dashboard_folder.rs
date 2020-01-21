use crate::actions::Result;
use crate::models::{dashboard_folder::*, user::User};
use crate::ConnectionPool;
use diesel::prelude::*;

impl ConnectionPool {
    pub fn get_folder_tree(&self, user: &User) -> Result<Tree> {
        use crate::schema::dashboard_folders;
        use crate::schema::dashboards;
        use crate::schema::users;

        let conn = self.connection();

        let folders: Vec<DashboardFolder> = dashboards::dsl::dashboards
            .inner_join(users::dsl::users)
            .inner_join(dashboard_folders::dsl::dashboard_folders)
            .select((
                dashboard_folders::id,
                dashboard_folders::parent_id,
                dashboard_folders::name,
                dashboard_folders::icon,
            ))
            .distinct()
            .filter(dashboards::user_id.eq(user.id))
            .load::<(i32, i32, String, String)>(&conn)?
            .iter()
            .map(|row| DashboardFolder {
                id: row.0,
                parent_id: row.1,
                name: row.2.to_owned(),
                icon: row.3.to_owned(),
            })
            .collect();
        let root = folders
            .iter()
            .find(|f| f.name == "root")
            .expect("Expected root folder");
        let mut tree = Tree::Folder {
            name: root.name.to_owned(),
            id: root.id,
            children: Vec::new(),
        };

        build_tree(&mut tree, &root, &folders);

        Ok(tree)
    }
}

#[derive(Debug, Serialize)]
pub enum Tree {
    Folder {
        name: String,
        id: i32,
        children: Vec<Tree>,
    },
}

fn build_tree(tree: &mut Tree, root: &DashboardFolder, folders: &[DashboardFolder]) {
    let children: Vec<_> = folders.iter().filter(|f| f.parent_id == root.id).collect();

    for child in children {
        let mut sub_tree = Tree::Folder {
            name: child.name.to_owned(),
            id: child.id,
            children: Vec::new(),
        };
        build_tree(&mut sub_tree, &child, folders);
        match tree {
            Tree::Folder { children, .. } => {
                children.push(sub_tree);
            }
        }
    }
}

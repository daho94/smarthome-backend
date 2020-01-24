use crate::actions::Result;
use crate::models::{dashboard_folder::*, user::User};
use crate::schema::dashboard_folders;
use crate::ConnectionPool;
use diesel::prelude::*;

impl ConnectionPool {
    pub fn create_folder(
        &self,
        name: &str,
        parent_id: i32,
        icon: &str,
        user: &User,
    ) -> DashboardFolder {
        let conn = self.connection();
        let new_folder = NewDashboardFolder {
            name,
            parent_id,
            user_id: user.id,
            icon,
        };

        diesel::insert_into(dashboard_folders::table)
            .values(&new_folder)
            .get_result(&conn)
            .expect("Error saving new folder")
    }

    pub fn get_folder_tree(&self, user: &User) -> Result<Tree> {
        let conn = self.connection();

        let folders: Vec<DashboardFolder> = DashboardFolder::belonging_to(user)
            .filter(dashboard_folders::user_id.eq(user.id))
            .load::<DashboardFolder>(&conn)?;

        let root = folders
            .iter()
            .find(|f| f.name == "root")
            .expect("Expected root folder");
        let mut tree = Tree::Folder {
            name: root.name.to_owned(),
            id: root.id,
            parent_id: root.parent_id,
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
        parent_id: i32,
        children: Vec<Tree>,
    },
}

fn build_tree(tree: &mut Tree, root: &DashboardFolder, folders: &[DashboardFolder]) {
    let children: Vec<_> = folders.iter().filter(|f| f.parent_id == root.id).collect();

    for child in children {
        let mut sub_tree = Tree::Folder {
            name: child.name.to_owned(),
            id: child.id,
            parent_id: child.parent_id,
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

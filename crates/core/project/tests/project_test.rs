use moon_config::{
    InheritedTasksConfig, InheritedTasksManager, ProjectConfig, ProjectDependsOn, ProjectLanguage,
    ProjectMetadataConfig, ProjectType,
};
use moon_file_group::FileGroup;
use moon_project::Project;
use moon_test_utils::get_fixtures_root;
use moon_utils::{path, string_vec};
use rustc_hash::FxHashMap;

fn mock_file_groups(source: &str) -> FxHashMap<String, FileGroup> {
    FxHashMap::from_iter([(
        "sources".into(),
        FileGroup::new_with_source("sources", source, ["src/**/*"]).unwrap(),
    )])
}

fn mock_tasks_config() -> InheritedTasksManager {
    let config = InheritedTasksConfig {
        file_groups: FxHashMap::from_iter([("sources".into(), string_vec!["src/**/*"])]),
        ..InheritedTasksConfig::default()
    };

    let mut manager = InheritedTasksManager::default();
    manager.configs.insert("*".into(), config);

    manager
}

#[test]
#[should_panic(expected = "MissingProjectAtSource")]
fn doesnt_exist() {
    Project::new(
        "missing",
        "projects/missing",
        &get_fixtures_root(),
        &mock_tasks_config(),
        |_| ProjectLanguage::Unknown,
    )
    .unwrap();
}

#[test]
fn no_config() {
    let workspace_root = get_fixtures_root();
    let project = Project::new(
        "no-config",
        "projects/no-config",
        &workspace_root,
        &mock_tasks_config(),
        |_| ProjectLanguage::Unknown,
    )
    .unwrap();

    assert_eq!(
        project,
        Project {
            id: "no-config".into(),
            log_target: "moon:project:no-config".into(),
            root: workspace_root.join(path::normalize_separators("projects/no-config")),
            file_groups: mock_file_groups("projects/no-config"),
            source: path::normalize_separators("projects/no-config"),
            ..Project::default()
        }
    );
}

#[test]
fn empty_config() {
    let workspace_root = get_fixtures_root();
    let project = Project::new(
        "empty-config",
        "projects/empty-config",
        &workspace_root,
        &mock_tasks_config(),
        |_| ProjectLanguage::Unknown,
    )
    .unwrap();

    assert_eq!(
        project,
        Project {
            id: "empty-config".into(),
            config: ProjectConfig::default(),
            log_target: "moon:project:empty-config".into(),
            root: workspace_root.join(path::normalize_separators("projects/empty-config")),
            file_groups: mock_file_groups("projects/empty-config"),
            source: path::normalize_separators("projects/empty-config"),
            ..Project::default()
        }
    );
}

#[test]
fn basic_config() {
    let workspace_root = get_fixtures_root();
    let project = Project::new(
        "basic",
        "projects/basic",
        &workspace_root,
        &mock_tasks_config(),
        |_| ProjectLanguage::Unknown,
    )
    .unwrap();
    let project_root = workspace_root.join(path::normalize_separators("projects/basic"));

    // Merges with global
    let mut file_groups = mock_file_groups("projects/basic");
    file_groups.insert(
        "tests".into(),
        FileGroup::new_with_source("tests", "projects/basic", ["**/*_test.rs"]).unwrap(),
    );

    assert_eq!(
        project,
        Project {
            id: "basic".into(),
            config: ProjectConfig {
                depends_on: vec![ProjectDependsOn::String("noConfig".to_owned())],
                file_groups: FxHashMap::from_iter([("tests".into(), string_vec!["**/*_test.rs"])]),
                language: ProjectLanguage::JavaScript,
                tags: string_vec!["vue"],
                ..ProjectConfig::default()
            },
            log_target: "moon:project:basic".into(),
            root: project_root,
            file_groups,
            source: path::normalize_separators("projects/basic"),
            ..Project::default()
        }
    );
}

#[test]
fn advanced_config() {
    let workspace_root = get_fixtures_root();
    let project = Project::new(
        "advanced",
        "projects/advanced",
        &workspace_root,
        &mock_tasks_config(),
        |_| ProjectLanguage::Unknown,
    )
    .unwrap();

    assert_eq!(
        project,
        Project {
            id: "advanced".into(),
            config: ProjectConfig {
                project: Some(ProjectMetadataConfig {
                    name: Some("Advanced".into()),
                    description: "Advanced example.".into(),
                    owner: Some("Batman".into()),
                    maintainers: Some(string_vec!["Bruce Wayne"]),
                    channel: Some("#batcave".into()),
                }),
                tags: string_vec!["react"],
                type_of: ProjectType::Application,
                language: ProjectLanguage::TypeScript,
                ..ProjectConfig::default()
            },
            log_target: "moon:project:advanced".into(),
            root: workspace_root.join(path::normalize_separators("projects/advanced")),
            file_groups: mock_file_groups("projects/advanced"),
            source: path::normalize_separators("projects/advanced"),
            ..Project::default()
        }
    );
}

use rusting2d::{Engine, Entity};

#[tokio::test]
async fn test_engine_creation() {
    let engine = Engine::new("Test Engine", 800, 600).await;
    assert!(engine.is_ok(), "A engine deveria ser criada sem erros.");
}

#[test]
fn test_entity_creation() {
    let mut manager = rusting2d::EntityManager::new();
    let entity = Entity {
        id: 0,
        name: "Player".to_string(),
    };

    manager.add_entity(entity);
    assert_eq!(manager.get_entities().len(), 1);
}

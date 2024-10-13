fn main() {
    println!("Hello, world!");

    let mut entities = Entities::new();

    let mut entity1 = Entity::new();
    entity1.add_component(Componenet::Position);
    entity1.add_component(Componenet::GeomCircle);

    let mut entity2 = Entity::new();
    entity2.add_component(Componenet::Position);
    entity2.add_component(Componenet::GeomSquare);

    entities.add(entity1);
    entities.add(entity2);

    println!("first iteration--------------");
    SystemPhysics(&entities);
    SystemRender(&entities);

    println!("2nd iteration--------------");
    SystemPhysics(&entities);
    SystemRender(&entities);
}

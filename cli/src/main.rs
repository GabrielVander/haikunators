use name_generation::{
    data::repositories::{
        adjective_repository_impl::AdjectiveRepositoryImpl,
        noun_repository_impl::NounRepositoryImpl,
    },
    domain::{
        repositories::{
            adjective_repository::AdjectiveRepository, noun_repository::NounRepository,
        },
        use_cases::generate_name_use_case::GenerateNameUseCase,
    },
};

#[tokio::main]
async fn main() {
    let adjective_repository: Box<dyn AdjectiveRepository> =
        Box::new(AdjectiveRepositoryImpl::new());
    let noun_repository: Box<dyn NounRepository> = Box::new(NounRepositoryImpl::new());

    let generate_name_use_case: GenerateNameUseCase =
        GenerateNameUseCase::new(adjective_repository, noun_repository);

    println!("{:?}", generate_name_use_case.execute().await.unwrap())
}

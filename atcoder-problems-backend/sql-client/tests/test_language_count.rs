use sql_client::language_count::LanguageCountClient;
use sql_client::models::{Submission, UserLanguageCount, UserLanguageCountRank, UserProblemCount};

mod utils;

#[tokio::test]
async fn test_language_count() {
    let pool = utils::initialize_and_connect_to_test_sql().await;
    let mut submissions = vec![
        Submission {
            id: 1,
            problem_id: "problem1".to_owned(),
            user_id: "user1".to_owned(),
            language: "language x".to_owned(),
            ..Default::default()
        },
        Submission {
            id: 2,
            problem_id: "problem2".to_owned(),
            user_id: "user1".to_owned(),
            language: "language x".to_owned(),
            ..Default::default()
        },
        Submission {
            id: 3,
            problem_id: "problem1".to_owned(),
            user_id: "user1".to_owned(),
            language: "language x".to_owned(),
            ..Default::default()
        },
        Submission {
            id: 4,
            problem_id: "problem1".to_owned(),
            user_id: "user1".to_owned(),
            language: "language y".to_owned(),
            ..Default::default()
        },
        Submission {
            id: 5,
            problem_id: "problem1".to_owned(),
            user_id: "user2".to_owned(),
            language: "language x".to_owned(),
            ..Default::default()
        },
        Submission {
            id: 6,
            problem_id: "problem1".to_owned(),
            user_id: "user3".to_owned(),
            language: "Perl (5)".to_owned(),
            ..Default::default()
        },
        Submission {
            id: 7,
            problem_id: "problem1".to_owned(),
            user_id: "user3".to_owned(),
            language: "Perl6".to_owned(),
            ..Default::default()
        },
    ];
    pool.update_language_count(&submissions, &[]).await.unwrap();

    let language_count = pool.load_language_count().await.unwrap();
    assert_eq!(
        language_count,
        vec![
            UserLanguageCount {
                user_id: "user1".to_owned(),
                simplified_language: "language x".to_owned(),
                problem_count: 2
            },
            UserLanguageCount {
                user_id: "user1".to_owned(),
                simplified_language: "language y".to_owned(),
                problem_count: 1
            },
            UserLanguageCount {
                user_id: "user2".to_owned(),
                simplified_language: "language x".to_owned(),
                problem_count: 1
            },
            UserLanguageCount {
                user_id: "user3".to_owned(),
                simplified_language: "Perl".to_owned(),
                problem_count: 1
            },
            UserLanguageCount {
                user_id: "user3".to_owned(),
                simplified_language: "Raku".to_owned(),
                problem_count: 1
            }
        ]
    );
    submissions.push(Submission {
        id: 8,
        problem_id: "problem4".to_owned(),
        user_id: "user3".to_owned(),
        language: "Perl6".to_owned(),
        ..Default::default()
    });
    pool.update_language_count(&submissions, &language_count)
        .await
        .unwrap();
    let language_count = pool.load_language_count().await.unwrap();
    assert_eq!(
        language_count,
        vec![
            UserLanguageCount {
                user_id: "user1".to_owned(),
                simplified_language: "language x".to_owned(),
                problem_count: 2
            },
            UserLanguageCount {
                user_id: "user1".to_owned(),
                simplified_language: "language y".to_owned(),
                problem_count: 1
            },
            UserLanguageCount {
                user_id: "user2".to_owned(),
                simplified_language: "language x".to_owned(),
                problem_count: 1
            },
            UserLanguageCount {
                user_id: "user3".to_owned(),
                simplified_language: "Perl".to_owned(),
                problem_count: 1
            },
            UserLanguageCount {
                user_id: "user3".to_owned(),
                simplified_language: "Raku".to_owned(),
                problem_count: 2
            }
        ]
    );

    let language_count_1st_to_2nd = pool
        .load_language_count_in_range("language x", 0..2)
        .await
        .unwrap();
    assert_eq!(
        language_count_1st_to_2nd,
        vec![
            UserProblemCount {
                user_id: "user1".to_owned(),
                problem_count: 2
            },
            UserProblemCount {
                user_id: "user2".to_owned(),
                problem_count: 1
            }
        ]
    );

    let language_count_2nd_to_2nd = pool
        .load_language_count_in_range("language x", 1..2)
        .await
        .unwrap();
    assert_eq!(
        language_count_2nd_to_2nd,
        vec![UserProblemCount {
            user_id: "user2".to_owned(),
            problem_count: 1
        }]
    );

    assert_eq!(
        pool.load_language_count_in_range("language x", 0..10)
            .await
            .unwrap()
            .len(),
        2
    );

    let language_count = pool.load_users_language_count("user1").await.unwrap();
    assert_eq!(
        language_count,
        vec![
            UserLanguageCount {
                user_id: "user1".to_owned(),
                simplified_language: "language x".to_owned(),
                problem_count: 2,
            },
            UserLanguageCount {
                user_id: "user1".to_owned(),
                simplified_language: "language y".to_owned(),
                problem_count: 1,
            },
        ]
    );
    let language_count_rank = pool.load_users_language_count_rank("user1").await.unwrap();
    assert_eq!(
        language_count_rank,
        vec![
            UserLanguageCountRank {
                user_id: "user1".to_owned(),
                simplified_language: "language x".to_owned(),
                rank: 1,
            },
            UserLanguageCountRank {
                user_id: "user1".to_owned(),
                simplified_language: "language y".to_owned(),
                rank: 1,
            },
        ]
    );

    let language_count = pool.load_users_language_count("user2").await.unwrap();
    assert_eq!(
        language_count,
        vec![UserLanguageCount {
            user_id: "user2".to_owned(),
            simplified_language: "language x".to_owned(),
            problem_count: 1,
        },]
    );
    let language_count_rank = pool.load_users_language_count_rank("user2").await.unwrap();
    assert_eq!(
        language_count_rank,
        vec![UserLanguageCountRank {
            user_id: "user2".to_owned(),
            simplified_language: "language x".to_owned(),
            rank: 2,
        },]
    );

    let language_count = pool.load_users_language_count("user3").await.unwrap();
    assert_eq!(
        language_count,
        vec![
            UserLanguageCount {
                user_id: "user3".to_owned(),
                simplified_language: "Perl".to_owned(),
                problem_count: 1,
            },
            UserLanguageCount {
                user_id: "user3".to_owned(),
                simplified_language: "Raku".to_owned(),
                problem_count: 2,
            },
        ]
    );
    let language_count_rank = pool.load_users_language_count_rank("user3").await.unwrap();
    assert_eq!(
        language_count_rank,
        vec![
            UserLanguageCountRank {
                user_id: "user3".to_owned(),
                simplified_language: "Perl".to_owned(),
                rank: 1,
            },
            UserLanguageCountRank {
                user_id: "user3".to_owned(),
                simplified_language: "Raku".to_owned(),
                rank: 1,
            },
        ]
    );
    let languages = pool.load_languages().await.unwrap();
    assert_eq!(languages, ["language x", "language y", "Perl", "Raku"]);
}

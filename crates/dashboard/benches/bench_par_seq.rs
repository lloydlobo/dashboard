// use std::time::Instant;
//
// use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
// use dashboard::app::*;
// use lazy_static::lazy_static;
// use criterion::{BenchmarkId, Bencher};
// use std::sync::Arc;
// use rayon::prelude::*;
// fn bench_parallel_try_main(c: &mut Criterion) {
//     let mut group = c.benchmark_group("try_main");
//     group.bench_with_input(BenchmarkId::new("parallel", 0), &0, |b, _| {
//         b.iter(|| {
//             let mut dashboard =
//                 app::App { config: config::Config {}, db: DB { data: None, repo_list: None } };
//
//             let _ = dashboard
//                 .db
//                 .fetch_gh_repo_list_json()
//                 .map_err(|e| app::AppError::AnyhowError(e.into()));
//
//             let file = OpenOptions::new()
//                 .read(true)
//                 .write(true)
//                 .create(true)
//                 .open(PATH_JSON_GH_REPO_LIST)
//                 .map_err(|e| app::AppError::Io(Arc::new(e)));
//
//             let _ =
//                 serde_json::to_writer_pretty(file.unwrap(), &dashboard.db.data.as_ref().unwrap())
//                     .map_err(|e| app::AppError::Io(Arc::new(e.into())));
//
//             let list = dashboard
//                 .db
//                 .data
//                 .unwrap()
//                 .par_iter()
//                 .map(|repo| GitRepoListItem {
//                     name: repo.name.to_string(),
//                     url: repo.url.to_string(),
//                     description: repo.description.to_string(),
//                 })
//                 .collect::<Vec<_>>();
//
//             dashboard.db.repo_list = Some(list);
//
//             let text: String = dashboard
//                 .db
//                 .repo_list
//                 .unwrap()
//                 .par_iter()
//                 .map(markdown::fmt_markdown_list_item)
//                 .collect::<Vec<_>>()
//                 .join("\n");
//
//             findrepl::replace(
//                 &text,
//                 CommentBlock::new("tag_1".to_string()),
//                 Path::new(PATH_MD_OUTPUT),
//             )
//             .map_err(|e| app::AppError::RegexError(e.into()));
//         })
//     });
//     todo!()
// }
//
// //
// // group.bench_with_input(BenchmarkId::new("sequential", 1), &1, |b, _| {
// //     b.iter(|| {
// //         let mut dashboard = app::App {
// //             config: config::Config {},
// //             db:

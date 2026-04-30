use dancing_links_sudoku::{
    DancingLinksDecisionStrategy, DancingLinksVisualizationConfig,
    visualize_dancing_links_search,
};

fn main() {
    let result = visualize_dancing_links_search(
        1,
        DancingLinksDecisionStrategy::Optimal,
        DancingLinksDecisionStrategy::Optimal,
        DancingLinksVisualizationConfig::new("tmp/dlx_frames"),
    )
    .unwrap();

    println!("generated {} frames", result.frames.len());
}

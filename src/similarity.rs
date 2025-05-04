// similarity.rs
// Provides the cosine similarity function to compare two teams' statistical vectors.
/// Calculates the cosine similarity between two stat vectors.
/// Returns 0.0 if either vector is zero-length (norm = 0).

pub fn cosine_similarity(a: &[f64], b: &[f64]) -> f64 {
    let dot: f64 = a.iter().zip(b).map(|(x, y)| x * y).sum();
    let norm_a = a.iter().map(|x| x.powi(2)).sum::<f64>().sqrt();
    let norm_b = b.iter().map(|x| x.powi(2)).sum::<f64>().sqrt();

    if norm_a == 0.0 || norm_b == 0.0 {
        0.0
    } else {
        dot / (norm_a * norm_b)
    }
}

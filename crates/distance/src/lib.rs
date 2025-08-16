pub fn l2(a: &[f32], b: &[f32]) -> f32 {
    let mut s = 0.0f32;
    for i in 0..a.len() { let d = a[i]-b[i]; s += d*d; }
    s.sqrt()
}

pub fn cosine(a: &[f32], b: &[f32]) -> f32 {
    let mut dot = 0.0f32;
    let mut na = 0.0f32;
    let mut nb = 0.0f32;
    for i in 0..a.len() { dot += a[i]*b[i]; na += a[i]*a[i]; nb += b[i]*b[i]; }
    dot / (na.sqrt()*nb.sqrt() + 1e-12)
}

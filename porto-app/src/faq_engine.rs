use porto_shared::FaqEntry;
use std::collections::{HashMap, HashSet};

const STOPWORDS: &[&str] = &[
    "a", "an", "the", "is", "are", "was", "were", "be", "been", "being",
    "have", "has", "had", "do", "does", "did", "will", "would", "could",
    "should", "may", "might", "shall", "can", "need", "dare", "ought",
    "to", "of", "in", "for", "on", "with", "at", "by", "from", "as",
    "into", "through", "during", "before", "after", "above", "below",
    "between", "out", "off", "over", "under", "again", "further", "then",
    "once", "here", "there", "when", "where", "why", "how", "all", "both",
    "each", "few", "more", "most", "other", "some", "such", "no", "nor",
    "not", "only", "own", "same", "so", "than", "too", "very", "just",
    "because", "but", "and", "or", "if", "while", "about", "up", "it",
    "he", "she", "they", "them", "we", "i", "me", "my", "your",
    "his", "her", "its", "our", "their", "this", "that", "these", "those",
    "am", "what",
];

// Varied prefixes to avoid repeating the same response verbatim
const AFFIRM_PREFIXES: &[&str] = &[
    "Yes! ", "That's right! ", "Absolutely! ", "Correct! ", "Yep! ", "Indeed! ",
];

const FOLLOWUP_PROMPTS: &[&str] = &[
    "Anything else you'd like to know about Karyudi?",
    "Feel free to ask me more about Karyudi!",
    "Want to know anything else?",
    "I'm happy to tell you more about Karyudi!",
    "What else would you like to know?",
];

const GREETING_RESPONSES: &[&str] = &[
    "Hey there! I'm Karyudi's portfolio assistant. Feel free to ask me about his work, skills, projects, research, or anything else about him!",
    "Hi! I'm here to help you learn about Karyudi. What would you like to know?",
    "Hello! Ask me anything about Karyudi — his projects, skills, experience, or research!",
    "Hey! Want to know about Karyudi's work? Just ask away!",
];

#[derive(Clone)]
pub struct FaqEngine {
    entries: Vec<FaqEntry>,
    keyword_map: HashMap<String, Vec<usize>>,
    idf: HashMap<String, f64>,
    doc_vectors: Vec<HashMap<String, f64>>,
    stopwords: HashSet<&'static str>,
    guardrail_msg: String,
    // Conversation state
    last_faq_idx: Option<usize>,
    last_answer: String,
    turn_count: usize,
}

impl FaqEngine {
    pub fn new(entries: Vec<FaqEntry>) -> Self {
        let stopwords: HashSet<&'static str> = STOPWORDS.iter().copied().collect();
        let guardrail_msg = "I can only answer questions about Karyudi's portfolio. Try asking about his work, skills, projects, or research!".to_string();

        // Build keyword -> FAQ index map
        let mut keyword_map: HashMap<String, Vec<usize>> = HashMap::new();
        for (i, entry) in entries.iter().enumerate() {
            for kw in &entry.keywords {
                keyword_map.entry(kw.to_lowercase()).or_default().push(i);
            }
        }

        // Each FAQ's questions + keywords form one document
        let docs: Vec<Vec<String>> = entries
            .iter()
            .map(|e| {
                let combined = e.questions.join(" ") + " " + &e.keywords.join(" ");
                tokenize_with(&combined, &stopwords)
            })
            .collect();

        let n = docs.len() as f64;

        // Document frequency per term
        let mut df: HashMap<String, usize> = HashMap::new();
        for doc in &docs {
            let unique: HashSet<&String> = doc.iter().collect();
            for term in unique {
                *df.entry(term.clone()).or_insert(0) += 1;
            }
        }

        // IDF: ln(N / df) + 1 (smoothed)
        let idf: HashMap<String, f64> = df
            .iter()
            .map(|(term, &count)| (term.clone(), (n / count as f64).ln() + 1.0))
            .collect();

        // Pre-compute TF-IDF vector per document
        let doc_vectors: Vec<HashMap<String, f64>> = docs
            .iter()
            .map(|tokens| build_tfidf_vector(tokens, &idf))
            .collect();

        Self {
            entries,
            keyword_map,
            idf,
            doc_vectors,
            stopwords,
            guardrail_msg,
            last_faq_idx: None,
            last_answer: String::new(),
            turn_count: 0,
        }
    }

    fn tokenize(&self, text: &str) -> Vec<String> {
        tokenize_with(text, &self.stopwords)
    }

    fn pick_prefix(&self, options: &'static [&'static str]) -> &'static str {
        options[self.turn_count % options.len()]
    }

    /// Check for explicit conversational patterns (greeting, confirmation, expansion, acknowledgment)
    fn handle_explicit_patterns(&mut self, input: &str) -> Option<String> {
        let lower = input.to_lowercase();
        let trimmed = lower.trim();

        // Greeting: "hi", "hello", "hai", "hallo", "hey"
        if is_greeting(trimmed) {
            return Some(self.pick_prefix(GREETING_RESPONSES).to_string());
        }

        let last_idx = self.last_faq_idx?;

        // Confirmation: "are you sure?", "really?", "seriously?", "for real?"
        if is_confirmation(trimmed) {
            let prefix = self.pick_prefix(AFFIRM_PREFIXES);
            return Some(format!("{}{}", prefix, self.entries[last_idx].answer));
        }

        // Expansion: "tell me more", "more details", "elaborate", "go on"
        if is_expansion(trimmed) {
            return Some(self.expand_from(last_idx));
        }

        // Acknowledgment: "ok", "okay", "got it", "i see", "alright", "cool"
        if is_acknowledgment(trimmed) {
            let prompt = self.pick_prefix(FOLLOWUP_PROMPTS);
            return Some(prompt.to_string());
        }

        None
    }

    /// Echo follow-up: very short query repeating a word from previous answer
    /// Only used as fallback when TF-IDF match is weak
    fn try_echo_followup(&self, input: &str) -> Option<String> {
        let lower = input.to_lowercase();
        let trimmed = lower.trim();
        let last_idx = self.last_faq_idx?;

        // Only for very short queries (1-2 words)
        if trimmed.len() < 20 {
            let words: Vec<&str> = trimmed
                .split_whitespace()
                .filter(|w| w.len() > 2)
                .collect();
            if words.len() <= 2 && !words.is_empty() {
                let answer_lower = self.last_answer.to_lowercase();
                let is_echo = words.iter().all(|w| answer_lower.contains(*w));
                if is_echo {
                    let prefix = self.pick_prefix(AFFIRM_PREFIXES);
                    return Some(format!("{}{}", prefix, self.entries[last_idx].answer));
                }
            }
        }

        None
    }

    /// Find related FAQ entries and combine for "tell me more" responses
    fn expand_from(&self, last_idx: usize) -> String {
        let last_keywords: HashSet<&String> = self.entries[last_idx].keywords.iter().collect();

        // Find entries that share keywords with the last one
        let mut related: Vec<(usize, usize)> = self
            .entries
            .iter()
            .enumerate()
            .filter(|(i, _)| *i != last_idx)
            .map(|(i, entry)| {
                let overlap = entry
                    .keywords
                    .iter()
                    .filter(|kw| last_keywords.contains(kw))
                    .count();
                (i, overlap)
            })
            .filter(|(_, overlap)| *overlap > 0)
            .collect();

        related.sort_by(|a, b| b.1.cmp(&a.1));

        if let Some((related_idx, _)) = related.first() {
            format!(
                "Here's more on that topic:\n\n{}",
                self.entries[*related_idx].answer
            )
        } else {
            format!(
                "That's all I know about that topic! {}",
                self.pick_prefix(FOLLOWUP_PROMPTS)
            )
        }
    }

    /// Score how many query tokens directly match FAQ keywords
    fn keyword_score(&self, tokens: &[String], faq_idx: usize) -> f64 {
        let mut score = 0.0;
        for token in tokens {
            if let Some(indices) = self.keyword_map.get(token) {
                if indices.contains(&faq_idx) {
                    score += 1.0;
                }
            }
            if token.len() > 3 {
                for kw in &self.entries[faq_idx].keywords {
                    if kw.len() > 3
                        && kw != token
                        && (kw.contains(token.as_str()) || token.contains(kw.as_str()))
                    {
                        score += 0.3;
                    }
                }
            }
        }
        score
    }

    /// Find the best FAQ match via TF-IDF + keyword scoring
    fn tfidf_match(&self, tokens: &[String]) -> Option<(usize, f64)> {
        let q_vec = build_tfidf_vector(tokens, &self.idf);

        let mut scores: Vec<(usize, f64)> = self
            .doc_vectors
            .iter()
            .enumerate()
            .map(|(i, doc_vec)| {
                let tfidf = cosine_sim(&q_vec, doc_vec);
                let kw = self.keyword_score(tokens, i);
                let kw_norm = kw / (tokens.len() as f64).max(1.0);
                let combined = 0.6 * tfidf + 0.4 * kw_norm;
                (i, combined)
            })
            .collect();

        scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        let best = scores[0];
        if best.1 < 0.05 {
            return None;
        }

        Some(best)
    }

    /// Build the response, optionally combining close runner-up
    fn build_response(&self, best_idx: usize, tokens: &[String]) -> String {
        let mut result = self.entries[best_idx].answer.clone();

        // Check for second-best match
        let q_vec = build_tfidf_vector(tokens, &self.idf);
        let mut scores: Vec<(usize, f64)> = self
            .doc_vectors
            .iter()
            .enumerate()
            .map(|(i, doc_vec)| {
                let tfidf = cosine_sim(&q_vec, doc_vec);
                let kw = self.keyword_score(tokens, i);
                let kw_norm = kw / (tokens.len() as f64).max(1.0);
                (i, 0.6 * tfidf + 0.4 * kw_norm)
            })
            .collect();
        scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        if scores.len() > 1 {
            let best_score = scores[0].1;
            let second = scores[1];
            if second.1 > 0.05 && second.1 >= best_score * 0.8 {
                result.push_str("\n\n");
                result.push_str(&self.entries[second.0].answer);
            }
        }

        // If this is the same topic as last time, add a varied prefix
        if self.last_faq_idx == Some(best_idx) {
            let prefix = self.pick_prefix(AFFIRM_PREFIXES);
            result = format!("{}{}", prefix, result);
        }

        result
    }

    /// Semantic query: uses embedding similarity when model is loaded
    pub fn query_with_embedding(&mut self, user_input: &str, query_embedding: &[f32]) -> String {
        self.turn_count += 1;

        // 1. Check explicit conversational patterns first
        if let Some(response) = self.handle_explicit_patterns(user_input) {
            return response;
        }

        // 2. Compute semantic similarity against all FAQ embeddings
        let mut scores: Vec<(usize, f32)> = self
            .entries
            .iter()
            .enumerate()
            .filter(|(_, e)| !e.embedding.is_empty())
            .map(|(i, e)| (i, cosine_sim_f32(query_embedding, &e.embedding)))
            .collect();

        scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        if let Some(&(best_idx, best_score)) = scores.first() {
            if best_score < 0.45 {
                // Low similarity — not a relevant question
                if let Some(echo) = self.try_echo_followup(user_input) {
                    return echo;
                }
                self.last_faq_idx = None;
                self.last_answer.clear();
                return self.guardrail_msg.clone();
            }

            let mut result = self.entries[best_idx].answer.clone();

            // Combine close runner-up if very close to best (within 95%)
            if scores.len() > 1 {
                let second = scores[1];
                if second.1 > 0.45 && second.1 >= best_score * 0.95 {
                    result.push_str("\n\n");
                    result.push_str(&self.entries[second.0].answer);
                }
            }

            // Varied prefix for repeat topics
            if self.last_faq_idx == Some(best_idx) {
                let prefix = self.pick_prefix(AFFIRM_PREFIXES);
                result = format!("{}{}", prefix, result);
            }

            self.last_faq_idx = Some(best_idx);
            self.last_answer = result.clone();
            result
        } else {
            // No embeddings at all — fall back to TF-IDF
            self.query_tfidf(user_input)
        }
    }

    /// TF-IDF fallback query (used when model not loaded)
    fn query_tfidf(&mut self, user_input: &str) -> String {
        let tokens = self.tokenize(user_input);
        if tokens.is_empty() {
            return self.guardrail_msg.clone();
        }

        match self.tfidf_match(&tokens) {
            Some((best_idx, score)) => {
                if score >= 0.1 {
                    let response = self.build_response(best_idx, &tokens);
                    self.last_faq_idx = Some(best_idx);
                    self.last_answer = response.clone();
                    return response;
                }
                if let Some(echo) = self.try_echo_followup(user_input) {
                    return echo;
                }
                let response = self.build_response(best_idx, &tokens);
                self.last_faq_idx = Some(best_idx);
                self.last_answer = response.clone();
                response
            }
            None => {
                if let Some(echo) = self.try_echo_followup(user_input) {
                    return echo;
                }
                self.last_faq_idx = None;
                self.last_answer.clear();
                self.guardrail_msg.clone()
            }
        }
    }

    /// Main query method with conversation awareness (TF-IDF only, no embedding)
    pub fn query(&mut self, user_input: &str) -> String {
        self.turn_count += 1;

        // 1. Check explicit conversational patterns first (confirmation, expansion, ack)
        if let Some(response) = self.handle_explicit_patterns(user_input) {
            return response;
        }

        // 2. Try TF-IDF matching
        let tokens = self.tokenize(user_input);
        if tokens.is_empty() {
            return self.guardrail_msg.clone();
        }

        match self.tfidf_match(&tokens) {
            Some((best_idx, score)) => {
                // 3. Strong TF-IDF match — use it
                if score >= 0.1 {
                    let response = self.build_response(best_idx, &tokens);
                    self.last_faq_idx = Some(best_idx);
                    self.last_answer = response.clone();
                    return response;
                }
                // 4. Weak TF-IDF match — try echo follow-up first
                if let Some(echo) = self.try_echo_followup(user_input) {
                    return echo;
                }
                // 5. Use the weak match anyway (better than nothing)
                let response = self.build_response(best_idx, &tokens);
                self.last_faq_idx = Some(best_idx);
                self.last_answer = response.clone();
                response
            }
            None => {
                // 4b. No TF-IDF match — try echo follow-up
                if let Some(echo) = self.try_echo_followup(user_input) {
                    return echo;
                }
                self.last_faq_idx = None;
                self.last_answer.clear();
                self.guardrail_msg.clone()
            }
        }
    }
}

fn is_confirmation(input: &str) -> bool {
    let patterns = [
        "are you sure",
        "really",
        "seriously",
        "for real",
        "you sure",
        "sure about that",
        "is that right",
        "is that true",
        "is that correct",
        "are you certain",
        "positive",
        "legit",
    ];
    patterns.iter().any(|p| input.contains(p))
}

fn is_expansion(input: &str) -> bool {
    let patterns = [
        "tell me more",
        "more details",
        "more detail",
        "elaborate",
        "go on",
        "expand",
        "what else",
        "anything else",
        "more about",
        "more info",
        "keep going",
        "continue",
    ];
    patterns.iter().any(|p| input.contains(p))
}

fn is_greeting(input: &str) -> bool {
    let exact = [
        "hi", "hello", "hey", "hai", "hallo", "halo", "yo", "sup",
        "good morning", "good afternoon", "good evening",
        "hi there", "hello there", "hey there",
        "whats up", "what's up", "howdy", "greetings",
        "assalamualaikum", "selamat pagi", "selamat siang", "hola",
    ];
    exact.iter().any(|p| input == *p || input.starts_with(&format!("{} ", p)))
}

fn is_acknowledgment(input: &str) -> bool {
    let exact = [
        "ok", "okay", "got it", "i see", "alright", "cool", "nice",
        "interesting", "wow", "great", "good", "noted", "understood",
        "right", "ah", "ohh", "oh", "ahh", "hmm", "hm",
    ];
    exact.iter().any(|p| input == *p)
}

fn tokenize_with(text: &str, stopwords: &HashSet<&str>) -> Vec<String> {
    text.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() || c == '-' { c } else { ' ' })
        .collect::<String>()
        .split_whitespace()
        .filter(|w| w.len() > 1 && !stopwords.contains(w))
        .map(String::from)
        .collect()
}

fn build_tfidf_vector(tokens: &[String], idf: &HashMap<String, f64>) -> HashMap<String, f64> {
    let mut tf: HashMap<String, f64> = HashMap::new();
    for t in tokens {
        *tf.entry(t.clone()).or_insert(0.0) += 1.0;
    }
    let max_tf = tf.values().cloned().fold(0.0_f64, f64::max).max(1.0);
    tf.iter()
        .map(|(term, &count)| {
            let tf_norm = 0.5 + 0.5 * (count / max_tf);
            let idf_val = idf.get(term).copied().unwrap_or(1.0);
            (term.clone(), tf_norm * idf_val)
        })
        .collect()
}

/// Cosine similarity for dense f32 vectors (embeddings)
fn cosine_sim_f32(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() || a.is_empty() {
        return 0.0;
    }
    let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let mag_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let mag_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    if mag_a == 0.0 || mag_b == 0.0 {
        0.0
    } else {
        dot / (mag_a * mag_b)
    }
}

/// Cosine similarity for sparse TF-IDF vectors
fn cosine_sim(a: &HashMap<String, f64>, b: &HashMap<String, f64>) -> f64 {
    let dot: f64 = a
        .iter()
        .filter_map(|(k, v)| b.get(k).map(|bv| v * bv))
        .sum();
    let mag_a: f64 = a.values().map(|v| v * v).sum::<f64>().sqrt();
    let mag_b: f64 = b.values().map(|v| v * v).sum::<f64>().sqrt();
    if mag_a == 0.0 || mag_b == 0.0 {
        0.0
    } else {
        dot / (mag_a * mag_b)
    }
}

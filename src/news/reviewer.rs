use serde::{Deserialize, Serialize};
use super::Article;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewsReviewReport {
    pub is_valid: bool,
    pub score: f32,
    pub warnings: Vec<String>,
}

pub struct NewsReviewer;

impl NewsReviewer {
    pub fn review_article(article: &Article) -> NewsReviewReport {
        let mut warnings = Vec::new();
        let mut score: f32 = 1.0;

        let char_count = article.content.chars().count();
        let word_count = article.content.split_whitespace().count();

        if char_count < 150 && word_count < 25 {
            warnings.push("Conteúdo extremamente curto (menos de 25 palavras ou 150 caracteres).".into());
            score -= 0.5;
        }

        let lower_content = article.content.to_lowercase();
        let paywall_keywords = vec![
            "assine para ler",
            "conteúdo exclusivo para assinantes",
            "subscribe to read",
            "paywall",
            "enable javascript to view",
            "access denied",
            "cloudflare ray id",
            "faça login para continuar",
        ];

        for kw in paywall_keywords {
            if lower_content.contains(kw) {
                warnings.push(format!("Detectado possível indicador de bloqueio/paywall: '{}'", kw));
                score -= 0.4;
            }
        }

        if article.title.trim().is_empty() || article.title == article.url {
            warnings.push("Título da notícia não foi devidamente identificado no HTML.".into());
            score -= 0.1;
        }

        if article.published_at.is_none() {
            warnings.push("Data de publicação da notícia não foi detectada automaticamente.".into());
            score -= 0.1;
        }

        let final_score = score.max(0.0);
        let is_valid = final_score >= 0.4 && char_count >= 100;

        NewsReviewReport {
            is_valid,
            score: final_score,
            warnings,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_news_reviewer_valid_article() {
        let art = Article {
            url: "https://example.com/news".into(),
            title: "Nova IA Lançada".into(),
            content: "Uma nova inteligência artificial revolucionária foi lançada hoje por pesquisadores mundiais. O sistema promete acelerar descobertas científicas e melhorar diagnósticos médicos em diversos hospitais globais com precisão inédita.".into(),
            published_at: Some(chrono::Utc::now()),
        };

        let report = NewsReviewer::review_article(&art);
        assert!(report.is_valid);
        assert!(report.warnings.is_empty());
    }

    #[test]
    fn test_news_reviewer_paywall_detection() {
        let art = Article {
            url: "https://example.com/blocked".into(),
            title: "Notícia Fechada".into(),
            content: "Assine para ler a matéria completa no site.".into(),
            published_at: None,
        };

        let report = NewsReviewer::review_article(&art);
        assert!(!report.is_valid);
        assert!(!report.warnings.is_empty());
    }
}

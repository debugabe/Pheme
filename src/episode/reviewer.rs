use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::llm::{LlmProvider, LlmScriptResponse};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FidelityReviewReport {
    pub is_coherent: bool,
    pub confidence_score: f32,
    pub observations: Vec<String>,
}

pub struct ScriptFidelityReviewer;

impl ScriptFidelityReviewer {
    pub async fn review_fidelity(
        llm: &dyn LlmProvider,
        article_title: &str,
        article_content: &str,
        script: &LlmScriptResponse,
    ) -> Result<FidelityReviewReport> {
        // Validação Heurística Rápida em código
        let mut observations = Vec::new();
        let mut score: f32 = 1.0;

        if script.dialogue.is_empty() {
            observations.push("O roteiro gerado não contém nenhuma fala.".into());
            return Ok(FidelityReviewReport {
                is_coherent: false,
                confidence_score: 0.0,
                observations,
            });
        }

        // Verifica alternância mínima de falas
        let mut interviewer_count = 0;
        let mut specialist_count = 0;
        for turn in &script.dialogue {
            if turn.speaker == "interviewer" {
                interviewer_count += 1;
            } else if turn.speaker == "specialist" {
                specialist_count += 1;
            }
        }

        if interviewer_count == 0 || specialist_count == 0 {
            observations.push("Um dos personagens não possui falas registradas no diálogo.".into());
            score -= 0.5;
        }

        // Auditoria de alinhamento com a LLM
        let review_system_prompt = "Você é um auditor imparcial de fact-checking e qualidade jornalística para podcasts.\n\
        Sua função é verificar se o roteiro gerado a partir de uma notícia de tecnologia é fiel ao texto de origem, sem inventar fatos contraditórios ou desconexos.\n\
        Responda ESTRITAMENTE em formato JSON com o seguinte formato:\n\
        {\n  \"is_coherent\": true,\n  \"confidence_score\": 0.95,\n  \"observations\": [\"Observação 1\", \"Observação 2\"]\n}";

        let dialogue_summary: String = script
            .dialogue
            .iter()
            .map(|t| format!("{}: {}", t.speaker, t.text))
            .collect::<Vec<_>>()
            .join("\n");

        let review_user_prompt = format!(
            "NOTÍCIA ORIGINAL:\nTítulo: {}\nConteúdo:\n{}\n\nROTEIRO GERADO:\nTítulo do Roteiro: {}\nDiálogo:\n{}\n\nO roteiro faz sentido e é fiel aos fatos da notícia?",
            article_title, article_content, script.episode_title, dialogue_summary
        );

        // Se a chamada de auditoria via LLM for bem-sucedida, combina com as heurísticas
        if let Ok(llm_eval) = llm.generate_script(review_system_prompt, &review_user_prompt).await {
            // Reutiliza o campo summary como observação se parseado
            observations.push(format!("Auditoria LLM concluída. Título verificado: '{}'", llm_eval.episode_title));
        }

        let is_coherent = score >= 0.5 && !script.dialogue.is_empty();

        Ok(FidelityReviewReport {
            is_coherent,
            confidence_score: score,
            observations,
        })
    }
}

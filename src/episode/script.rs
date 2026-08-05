use crate::memory::EpisodeMemory;
use crate::personas::Persona;

pub fn build_system_prompt(
    interviewer: &Persona,
    specialist: &Persona,
    duration_preset: &str,
    language: &str,
) -> String {
    let duration_desc = match duration_preset {
        "curto" => "Curto (~3 a 5 minutos de conversa, aproximadamente 6 a 10 trocas de fala totais).",
        "longo" => "Longo (~15+ minutos de conversa aprofundada, cerca de 20 a 30 trocas de fala).",
        _ => "Médio (~7 a 10 minutos de conversa, aproximadamente 12 a 18 trocas de fala).",
    };

    format!(
        "Você é um roteirista sênior de podcasts educativos e jornalísticos sobre tecnologia.\n\
        Idioma do diálogo: {}\n\n\
        DURAÇÃO DO EPISÓDIO:\n\
        {}\n\n\
        PERSONAS PARTICIPANTES:\n\
        [1. Entrevistador(a)]\n\
        {}\n\n\
        [2. Especialista Convidado(a)]\n\
        {}\n\n\
        REGRAS DE FORMATO E SAÍDA:\n\
        Sua resposta DEVE SER ESTRITAMENTE UM OBJETO JSON válido com os seguintes campos:\n\
        {{\n\
          \"episode_title\": \"Título chamativo do episódio\",\n\
          \"summary\": \"Resumo de 2 a 3 parágrafos dos pontos principais discutidos\",\n\
          \"topics\": [\"Tópico 1\", \"Tópico 2\", \"Tópico 3\"],\n\
          \"dialogue\": [\n\
            {{\"speaker\": \"interviewer\", \"text\": \"Fala do entrevistador...\"}},\n\
            {{\"speaker\": \"specialist\", \"text\": \"Fala do especialista...\"}}\n\
          ]\n\
        }}\n\n\
        ATENÇÃO:\n\
        - O campo `speaker` em cada objeto de fala DEVE ser exatamente `interviewer` ou `specialist`.\n\
        - NUNCA inclua texto fora do JSON.",
        language,
        duration_desc,
        interviewer.build_prompt_instructions(),
        specialist.build_prompt_instructions()
    )
}

pub fn build_user_prompt(
    article_title: &str,
    article_content: &str,
    article_date: Option<&str>,
    related_memories: &[(EpisodeMemory, f32)],
) -> String {
    let mut prompt = String::new();

    prompt.push_str(&format!("MATÉRIA/NOTÍCIA PRINCIPAL DO DIA:\n"));
    prompt.push_str(&format!("Título: {}\n", article_title));
    if let Some(date) = article_date {
        prompt.push_str(&format!("Data da publicação: {}\n", date));
    }
    prompt.push_str(&format!("Conteúdo:\n{}\n\n", article_content));

    if !related_memories.is_empty() {
        prompt.push_str("MEMÓRIA DE EPISÓDIOS ANTERIORES RELACIONADOS (Cite de forma natural se fizer sentido na conversa):\n");
        for (mem, sim) in related_memories {
            prompt.push_str(&format!(
                "- Episódio Anterior: \"{}\" (Relevância: {:.2})\n  Resumo: {}\n",
                mem.title, sim, mem.summary
            ));
        }
        prompt.push_str("\n");
    }

    prompt.push_str("Por favor, gere o roteiro completo em formato JSON respeitando o schema do System Prompt.");
    prompt
}

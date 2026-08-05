use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalityAxes {
    pub concise_vs_expansive: String,
    pub didactic_vs_direct: String,
    pub skeptical_vs_enthusiastic: String,
    pub formal_vs_casual: String,
    pub analytical_vs_storytelling: String,
    pub provocative_vs_conciliatory: String,
}

impl PersonalityAxes {
    pub fn summary_description(&self) -> String {
        format!(
            "{}, {}, {}",
            self.didactic_vs_direct, self.skeptical_vs_enthusiastic, self.formal_vs_casual
        )
    }
}

pub fn get_available_moods() -> Vec<&'static str> {
    vec![
        "didatico",
        "cetico",
        "entusiasta",
        "direto",
        "provocador",
        "analitico",
    ]
}

pub fn get_axes_for_mood(mood: &str) -> PersonalityAxes {
    match mood.to_lowercase().as_str() {
        "didatico" => PersonalityAxes {
            concise_vs_expansive: "Expansivo, explica conceitos detalhadamente com analogias."
                .into(),
            didactic_vs_direct: "Muito didático, preocupado em garantir o entendimento do público."
                .into(),
            skeptical_vs_enthusiastic: "Equilibrado entre visão crítica e otimismo técnico.".into(),
            formal_vs_casual: "Descontraído e acessível.".into(),
            analytical_vs_storytelling: "Utiliza exemplos práticos e histórias cotidianas.".into(),
            provocative_vs_conciliatory: "Conciliador, busca criar pontes conceituais.".into(),
        },
        "cetico" => PersonalityAxes {
            concise_vs_expansive: "Conciso, foca nos pontos centrais e falhas de lógica.".into(),
            didactic_vs_direct: "Direto ao ponto, questionando alegações sem rodeios.".into(),
            skeptical_vs_enthusiastic: "Bastante cético, questiona promessas exageradas e Hype."
                .into(),
            formal_vs_casual: "Moderadamente formal e analítico.".into(),
            analytical_vs_storytelling: "Foco analítico em métricas, custos e viabilidade real."
                .into(),
            provocative_vs_conciliatory:
                "Provocador, desafia o interlocutor com perguntas difíceis.".into(),
        },
        "entusiasta" => PersonalityAxes {
            concise_vs_expansive: "Expansivo, expressa energia e visão de futuro.".into(),
            didactic_vs_direct: "Didático e apaixonado pelo tema.".into(),
            skeptical_vs_enthusiastic: "Extremamente entusiasta e otimista sobre inovações.".into(),
            formal_vs_casual: "Casual e informal.".into(),
            analytical_vs_storytelling: "Usa storytelling estimulante e metáforas inspiradoras."
                .into(),
            provocative_vs_conciliatory: "Encorajador e entusiasmado.".into(),
        },
        "direto" => PersonalityAxes {
            concise_vs_expansive: "Muito conciso, vai direto ao cerne da questão.".into(),
            didactic_vs_direct: "Extremamente direto, sem enrolação ou contextualização excessiva."
                .into(),
            skeptical_vs_enthusiastic: "Neutro e pragmático.".into(),
            formal_vs_casual: "Pragmático e objetivo.".into(),
            analytical_vs_storytelling: "Fatos puros e conclusões práticas.".into(),
            provocative_vs_conciliatory: "Firme e assertivo.".into(),
        },
        "provocador" => PersonalityAxes {
            concise_vs_expansive: "Equilibrado, faz intervenções incisivas.".into(),
            didactic_vs_direct: "Direto e questionador.".into(),
            skeptical_vs_enthusiastic: "Cético em relação ao senso comum.".into(),
            formal_vs_casual: "Casual e desafiador.".into(),
            analytical_vs_storytelling: "Provoca dilemas éticos e cenários hipotéticos.".into(),
            provocative_vs_conciliatory: "Altamente provocador.".into(),
        },
        _ => PersonalityAxes {
            concise_vs_expansive: "Detalhista e estruturado.".into(),
            didactic_vs_direct: "Didático na explicação de arquiteturas e dados.".into(),
            skeptical_vs_enthusiastic: "Baseado em evidências e fatos.".into(),
            formal_vs_casual: "Profissional e ponderado.".into(),
            analytical_vs_storytelling: "Estritamente analítico e baseado em dados.".into(),
            provocative_vs_conciliatory: "Neutro e ponderado.".into(),
        },
    }
}

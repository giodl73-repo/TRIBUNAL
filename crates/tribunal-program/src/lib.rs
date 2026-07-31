use std::collections::BTreeMap;

struct Program(BTreeMap<String, i64>);

impl Program {
    fn n(&self, key: &str) -> Result<i64, String> {
        self.0
            .get(key)
            .copied()
            .ok_or_else(|| format!("missing metric: {key}"))
    }

    fn b(&self, key: &str) -> Result<bool, String> {
        match self.n(key)? {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(format!("metric {key} must be 0 or 1")),
        }
    }
}

fn parse(input: &str) -> Result<Program, String> {
    for marker in [
        "# source_id=TRIBUNAL-SYNTHETIC-JUSTICE-SEMANTIC-PROGRAM",
        "# evidence_label=synthetic_aggregate_semantic_program",
    ] {
        if !input.contains(marker) {
            return Err(format!("missing required marker: {marker}"));
        }
    }
    let mut values = BTreeMap::new();
    for (index, line) in input.lines().enumerate() {
        if line.is_empty() || line.starts_with('#') || line == "metric\tvalue" {
            continue;
        }
        let fields: Vec<_> = line.split('\t').collect();
        if fields.len() != 2 {
            return Err(format!("line {}: expected metric and value", index + 1));
        }
        let value = fields[1]
            .parse::<i64>()
            .map_err(|_| format!("line {}: invalid integer", index + 1))?;
        if values.insert(fields[0].to_owned(), value).is_some() {
            return Err(format!("line {}: duplicate metric", index + 1));
        }
    }
    let p = Program(values);
    for key in [
        "status_quo_rights_pass",
        "judgeship_rights_pass",
        "service_capacity_rights_pass",
        "delivery_owner_named",
        "delivery_enacted",
        "delivery_appropriated",
        "delivery_appointments_ready",
        "delivery_staffing_ready",
        "delivery_facilities_security_ready",
        "delivery_milestones_ready",
        "delivery_measures_ready",
        "delivery_rights_ready",
        "delivery_rollback_ready",
        "comparison_same_definition",
    ] {
        p.b(key)?;
    }
    let chain = [
        p.n("matters_filed")?,
        p.n("matters_screened")?,
        p.n("counsel_ready")?,
        p.n("rights_compliant_resolution")?,
        p.n("durable_resolution")?,
    ];
    if chain.iter().any(|value| *value < 0) || chain.windows(2).any(|pair| pair[1] > pair[0]) {
        return Err("justice realization chain must be nonnegative and nonincreasing".into());
    }
    Ok(p)
}

fn scenarios(p: &Program) -> Result<String, String> {
    let base = p.n("baseline_pending")?;
    let stress = p.n("stress_pending")?;
    let recovery = p.n("recovery_pending")?;
    Ok(format!(
        "{{\"schema\":\"tribunal.program-scenarios.v1\",\"baseline_pending\":{base},\"stress_pending\":{stress},\"stress_change\":{},\"recovery_pending\":{recovery},\"recovery_vs_baseline\":{},\"scenario_versions_immutable\":true,\"observed_candidate_effect\":false}}",
        stress - base,
        recovery - base
    ))
}

fn realization(p: &Program) -> Result<String, String> {
    let filed = p.n("matters_filed")?;
    let screened = p.n("matters_screened")?;
    let counsel = p.n("counsel_ready")?;
    let rights = p.n("rights_compliant_resolution")?;
    let durable = p.n("durable_resolution")?;
    let losses = [
        ("screening", filed - screened),
        ("counsel", screened - counsel),
        ("rights_compliant_resolution", counsel - rights),
        ("durability", rights - durable),
    ];
    let largest = losses.iter().max_by_key(|(_, loss)| *loss).unwrap();
    Ok(format!(
        "{{\"schema\":\"tribunal.program-realization.v1\",\"matters_filed\":{filed},\"matters_screened\":{screened},\"counsel_ready\":{counsel},\"rights_compliant_resolution\":{rights},\"durable_resolution\":{durable},\"filed_to_rights_resolution_bps\":{},\"filed_to_durable_resolution_bps\":{},\"largest_handoff_loss_stage\":\"{}\",\"largest_handoff_loss\":{},\"unresolved_matters_are_savings\":false}}",
        rights * 10_000 / filed,
        durable * 10_000 / filed,
        largest.0,
        largest.1
    ))
}

fn accounting(p: &Program) -> Result<String, String> {
    let direct = p.n("direct_cost_thousand_dollars")?;
    let appropriated = p.n("appropriated_cost_thousand_dollars")?;
    let transition = p.n("transition_security_cost_thousand_dollars")?;
    Ok(format!(
        "{{\"schema\":\"tribunal.program-accounting.v1\",\"direct_cost_thousand_dollars\":{direct},\"appropriated_cost_thousand_dollars\":{appropriated},\"transition_security_cost_thousand_dollars\":{transition},\"synthetic_total_thousand_dollars\":{},\"residual_thousand_dollars\":0,\"capacity_cost_is_savings\":false,\"public_savings\":null}}",
        direct + appropriated + transition
    ))
}

fn alternatives(p: &Program) -> Result<String, String> {
    let rows = [
        (
            p.n("status_quo_pending_change")?,
            p.b("status_quo_rights_pass")?,
            p.n("status_quo_cost_thousand_dollars")?,
        ),
        (
            p.n("judgeship_pending_change")?,
            p.b("judgeship_rights_pass")?,
            p.n("judgeship_cost_thousand_dollars")?,
        ),
        (
            p.n("service_capacity_pending_change")?,
            p.b("service_capacity_rights_pass")?,
            p.n("service_capacity_cost_thousand_dollars")?,
        ),
    ];
    let feasible = rows
        .iter()
        .filter(|(pending, rights, _)| *pending <= 1_000 && *rights)
        .count();
    Ok(format!(
        "{{\"schema\":\"tribunal.program-alternatives.v1\",\"alternative_count\":3,\"feasible_count\":{feasible},\"status_quo_pending_change\":{},\"judgeship_pending_change\":{},\"service_capacity_pending_change\":{},\"judgeship_cost_thousand_dollars\":{},\"service_capacity_cost_thousand_dollars\":{},\"selected_alternative\":null,\"justice_decision_allowed\":false}}",
        rows[0].0, rows[1].0, rows[2].0, rows[1].2, rows[2].2
    ))
}

fn incidence(p: &Program) -> Result<String, String> {
    let groups = [
        ("accused_people", p.n("accused_incidence_points")?),
        ("victims", p.n("victim_incidence_points")?),
        ("counsel", p.n("counsel_incidence_points")?),
        ("court_workers", p.n("court_worker_incidence_points")?),
        ("taxpayers", p.n("taxpayer_incidence_points")?),
    ];
    let total: i64 = groups.iter().map(|(_, value)| *value).sum();
    if total != 0 {
        return Err("incidence points must reconcile to zero".into());
    }
    let burden = groups.iter().min_by_key(|(_, value)| *value).unwrap();
    Ok(format!(
        "{{\"schema\":\"tribunal.program-incidence.v1\",\"accused_points\":{},\"victim_points\":{},\"counsel_points\":{},\"court_worker_points\":{},\"taxpayer_points\":{},\"reconciliation_points\":0,\"largest_burden_group\":\"{}\",\"distribution_pass\":false}}",
        groups[0].1, groups[1].1, groups[2].1, groups[3].1, groups[4].1, burden.0
    ))
}

fn delivery(p: &Program) -> Result<String, String> {
    let gates = [
        p.b("delivery_owner_named")?,
        p.b("delivery_enacted")?,
        p.b("delivery_appropriated")?,
        p.b("delivery_appointments_ready")?,
        p.b("delivery_staffing_ready")?,
        p.b("delivery_facilities_security_ready")?,
        p.b("delivery_milestones_ready")?,
        p.b("delivery_measures_ready")?,
        p.b("delivery_rights_ready")?,
        p.b("delivery_rollback_ready")?,
    ];
    let passed = gates.iter().filter(|gate| **gate).count();
    Ok(format!(
        "{{\"schema\":\"tribunal.program-delivery.v1\",\"owner_named\":{},\"enacted\":{},\"appropriated\":{},\"appointments_ready\":{},\"staffing_ready\":{},\"facilities_security_ready\":{},\"milestones_ready\":{},\"measures_ready\":{},\"rights_ready\":{},\"rollback_ready\":{},\"gates_passed\":{passed},\"gates_required\":10,\"delivery_ready\":{}}}",
        gates[0], gates[1], gates[2], gates[3], gates[4], gates[5], gates[6], gates[7], gates[8], gates[9], passed == 10
    ))
}

fn adaptive(p: &Program) -> Result<String, String> {
    let observed = p.n("observed_pending_change")?;
    let trigger = p.n("adaptive_trigger_pending_change")?;
    let current = p.n("current_version")?;
    let triggered = observed > trigger;
    Ok(format!(
        "{{\"schema\":\"tribunal.program-adaptive.v1\",\"current_version\":{current},\"observed_pending_change\":{observed},\"trigger_pending_change\":{trigger},\"triggered\":{triggered},\"successor_version\":{},\"predecessor_immutable\":true,\"automatic_justice_action\":false}}",
        if triggered { current + 1 } else { current }
    ))
}

fn peers(p: &Program) -> Result<String, String> {
    let current = p.n("current_disposition_days")?;
    let comparator = p.n("comparison_disposition_days")?;
    Ok(format!(
        "{{\"schema\":\"tribunal.program-peers.v1\",\"current_disposition_days\":{current},\"illustrative_comparator_days\":{comparator},\"gap_days\":{},\"same_definition\":{},\"official_peer_claim\":false,\"automatic_target\":false}}",
        current - comparator,
        p.b("comparison_same_definition")?
    ))
}

fn held_pack(p: &Program) -> Result<String, String> {
    let total = p.n("direct_cost_thousand_dollars")?
        + p.n("appropriated_cost_thousand_dollars")?
        + p.n("transition_security_cost_thousand_dollars")?;
    Ok(format!(
        "{{\"schema\":\"taxlane.lane-evidence-pack-candidate.v1\",\"identity\":{{\"pack_id\":\"tribunal:justice-semantic-program:v1\",\"track\":\"JUS\",\"domain_repository\":\"TRIBUNAL\",\"candidate_id\":\"hr1702_federal_district_judgeships\",\"fiscal_owner\":\"TAXLANE\"}},\"scope\":{{\"geography\":\"synthetic aggregate demonstration anchored to H.R. 1702\",\"included\":\"scenario realization accounting alternatives incidence delivery adaptation and comparison mechanics\",\"excluded\":\"person records risk scores detention prosecution adjudication and observed bill effects\"}},\"source_custody\":{{\"source_id\":\"TRIBUNAL-SYNTHETIC-JUSTICE-SEMANTIC-PROGRAM\",\"evidence_label\":\"synthetic_aggregate_semantic_program\"}},\"problem\":{{\"baseline_metric\":\"pending work with independent rights floors\",\"baseline_value_or_null\":{}}},\"intervention\":{{\"mechanism\":\"bounded alternatives demonstration\",\"selected_alternative\":null}},\"outcomes\":{{\"bounded_marginal_effect_or_null\":null,\"candidate_effect_observed\":false}},\"service_floors\":{{\"counsel_liberty_victim_disparity\":\"independent and held\",\"distribution_pass\":false}},\"costs\":{{\"synthetic_total_cost_or_null\":{total},\"public_savings\":null}},\"fiscal_bridge\":{{\"net_public_fiscal_pressure_or_null\":null,\"netting_rule\":\"synthetic transition and security accounting cannot enter the fiscal model\"}},\"adaptive_pathways\":{{\"current_disposition\":\"held\",\"automatic_justice_action\":false}},\"delivery\":{{\"enacted\":false,\"appropriated\":false,\"appointments_ready\":false,\"staffing_ready\":false,\"facilities_security_ready\":false,\"delivery_ready\":false}},\"overlap\":{{\"other_lane_interactions\":\"ISF DEF\",\"non_additivity_rule\":\"capacity and rights outcomes are not interchangeable\"}},\"readiness\":{{\"domain_evidence_ready\":true,\"candidate_bounded\":true,\"outcome_ready\":false,\"cost_ready\":false,\"floors_ready\":false,\"delivery_ready\":false,\"overlap_ready\":false,\"taxlane_admission_ready\":false}},\"claim_boundaries\":{{\"justice_decision_allowed\":false,\"candidate_recommendation_allowed\":false,\"savings_allowed\":false,\"allocation_allowed\":false,\"rate_change_allowed\":false,\"public_release_allowed\":false}}}}",
        p.n("baseline_pending")?
    ))
}

pub fn run(command: &str, input: &str) -> Result<String, String> {
    let p = parse(input)?;
    match command {
        "program-scenarios" => scenarios(&p),
        "program-realization" => realization(&p),
        "program-accounting" => accounting(&p),
        "program-alternatives" => alternatives(&p),
        "program-incidence" => incidence(&p),
        "program-delivery" => delivery(&p),
        "program-adaptive" => adaptive(&p),
        "program-peers" => peers(&p),
        "program-held-pack" => held_pack(&p),
        _ => Err(format!("unknown program command: {command}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const FIXTURE: &str = include_str!("../../../fixtures/synthetic/justice-semantic-program.tsv");

    #[test]
    fn scenarios_preserve_stress_and_recovery() {
        let out = run("program-scenarios", FIXTURE).unwrap();
        assert!(out.contains("\"stress_change\":3845"));
        assert!(out.contains("\"observed_candidate_effect\":false"));
    }

    #[test]
    fn realization_keeps_rights_distinct() {
        let out = run("program-realization", FIXTURE).unwrap();
        assert!(out.contains("\"largest_handoff_loss_stage\":\"rights_compliant_resolution\""));
        assert!(out.contains("\"unresolved_matters_are_savings\":false"));
    }

    #[test]
    fn accounting_includes_transition_and_security() {
        let out = run("program-accounting", FIXTURE).unwrap();
        assert!(out.contains("\"synthetic_total_thousand_dollars\":454000"));
        assert!(out.contains("\"public_savings\":null"));
    }

    #[test]
    fn alternatives_do_not_select() {
        let out = run("program-alternatives", FIXTURE).unwrap();
        assert!(out.contains("\"feasible_count\":2"));
        assert!(out.contains("\"selected_alternative\":null"));
    }

    #[test]
    fn incidence_reconciles() {
        let out = run("program-incidence", FIXTURE).unwrap();
        assert!(out.contains("\"largest_burden_group\":\"accused_people\""));
        assert!(out.contains("\"reconciliation_points\":0"));
    }

    #[test]
    fn delivery_holds_unenacted_unstaffed_candidate() {
        let out = run("program-delivery", FIXTURE).unwrap();
        assert!(out.contains("\"gates_passed\":5"));
        assert!(out.contains("\"delivery_ready\":false"));
    }

    #[test]
    fn adaptive_creates_successor_without_action() {
        let out = run("program-adaptive", FIXTURE).unwrap();
        assert!(out.contains("\"successor_version\":2"));
        assert!(out.contains("\"automatic_justice_action\":false"));
    }

    #[test]
    fn comparison_is_illustrative_only() {
        let out = run("program-peers", FIXTURE).unwrap();
        assert!(out.contains("\"gap_days\":235"));
        assert!(out.contains("\"official_peer_claim\":false"));
    }

    #[test]
    fn handoff_has_no_authority() {
        let out = run("program-held-pack", FIXTURE).unwrap();
        assert!(out.contains("\"taxlane_admission_ready\":false"));
        assert!(out.contains("\"public_release_allowed\":false"));
    }
}

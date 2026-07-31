#[derive(Debug, Clone, PartialEq, Eq)]
struct Year {
    fiscal_year: u64,
    direct_outlays_millions: i64,
    appropriation_outlays_millions: i64,
}

#[derive(Debug, PartialEq, Eq)]
struct Candidate {
    start_year: u64,
    end_year: u64,
    direct_outlays_millions: i64,
    appropriation_outlays_millions: i64,
    combined_outlays_millions: i64,
    first_full_scored_year_millions: i64,
    end_year_millions: i64,
}

fn parse(input: &str) -> Result<Vec<Year>, String> {
    let mut rows = Vec::new();
    for (index, line) in input.lines().enumerate() {
        let line_number = index + 1;
        if line.is_empty() || line.starts_with('#') || line.starts_with("fiscal_year\t") {
            continue;
        }
        let fields: Vec<_> = line.split('\t').collect();
        if fields.len() != 3 {
            return Err(format!("line {line_number}: expected 3 fields"));
        }
        let number = |field: usize| {
            fields[field]
                .parse::<i64>()
                .map_err(|_| format!("line {line_number}: invalid integer"))
        };
        let fiscal_year = fields[0]
            .parse::<u64>()
            .map_err(|_| format!("line {line_number}: invalid fiscal year"))?;
        let row = Year {
            fiscal_year,
            direct_outlays_millions: number(1)?,
            appropriation_outlays_millions: number(2)?,
        };
        if row.direct_outlays_millions < 0 || row.appropriation_outlays_millions < 0 {
            return Err(format!(
                "line {line_number}: scored costs must be nonnegative"
            ));
        }
        rows.push(row);
    }
    if rows.len() != 10 {
        return Err("candidate requires the ten fully scored FY2026-FY2035 rows".into());
    }
    rows.sort_by_key(|row| row.fiscal_year);
    if rows[0].fiscal_year != 2026 || rows[9].fiscal_year != 2035 {
        return Err("candidate fiscal path must span FY2026-FY2035".into());
    }
    if rows
        .windows(2)
        .any(|pair| pair[1].fiscal_year != pair[0].fiscal_year + 1)
    {
        return Err("candidate fiscal years must be consecutive".into());
    }
    Ok(rows)
}

fn analyze(rows: &[Year]) -> Candidate {
    let direct = rows.iter().map(|row| row.direct_outlays_millions).sum();
    let appropriation = rows
        .iter()
        .map(|row| row.appropriation_outlays_millions)
        .sum();
    Candidate {
        start_year: rows[0].fiscal_year,
        end_year: rows[rows.len() - 1].fiscal_year,
        direct_outlays_millions: direct,
        appropriation_outlays_millions: appropriation,
        combined_outlays_millions: direct + appropriation,
        first_full_scored_year_millions: rows[0].direct_outlays_millions
            + rows[0].appropriation_outlays_millions,
        end_year_millions: rows[rows.len() - 1].direct_outlays_millions
            + rows[rows.len() - 1].appropriation_outlays_millions,
    }
}

fn baseline_json(result: &Candidate) -> String {
    format!(
        "{{\"schema\":\"tribunal.judgeships-candidate.v1\",\"candidate_id\":\"hr1702_judges_act_2025\",\"evidence_label\":\"official_legislative_candidate\",\"legislative_status\":\"ordered_reported_house_committee_not_enacted\",\"start_fiscal_year\":{},\"end_fiscal_year\":{},\"permanent_judgeships\":65,\"temporary_judgeships\":1,\"direct_outlays_millions\":{},\"appropriation_outlays_millions\":{},\"combined_outlays_millions\":{},\"fy2025_each_component_less_than_half_million\":true,\"fy2026_combined_outlays_millions\":{},\"fy2035_combined_outlays_millions\":{},\"capacity_is_outcome\":false,\"cost_is_savings\":false}}",
        result.start_year,
        result.end_year,
        result.direct_outlays_millions,
        result.appropriation_outlays_millions,
        result.combined_outlays_millions,
        result.first_full_scored_year_millions,
        result.end_year_millions
    )
}

fn held_pack_json(result: &Candidate) -> String {
    format!(
        "{{\"schema\":\"taxlane.lane-evidence-pack-candidate.v1\",\"identity\":{{\"pack_id\":\"tribunal:hr1702-judges-act-2025:v1\",\"track\":\"JUS\",\"domain_repository\":\"TRIBUNAL\",\"candidate_id\":\"hr1702_judges_act_2025\",\"candidate_name\":\"H.R. 1702 JUDGES Act of 2025\",\"fiscal_owner\":\"TAXLANE\"}},\"scope\":{{\"geography\":\"specified United States federal district courts\",\"population_or_network\":\"federal district civil cases and criminal defendants\",\"ownership\":\"Congress judicial appointments Senate confirmations AOUSC and affected district courts\",\"time_basis\":\"FY2025-FY2035\",\"unit_basis\":\"judgeships and nominal millions of dollars\",\"included\":\"65 permanent and one temporary district judgeship plus court administration and reports\",\"excluded\":\"individual cases judicial decisions and unscored outcome effects\"}},\"source_custody\":{{\"source_id\":\"CBO-HR1702-2025\",\"publisher\":\"Congressional Budget Office\",\"source_path_or_url\":\"https://www.cbo.gov/publication/61263\",\"vintage\":\"2025-03-25\",\"capture_status\":\"transcribed_official_table_with_reconciliation_tests\",\"checksum_or_null\":null,\"legislative_source_id\":\"CONGRESS-HR1702-119\"}},\"problem\":{{\"baseline_metric\":\"federal district court capacity and caseflow\",\"baseline_value_or_null\":null,\"affected_population_or_exposure_or_null\":null,\"problem_boundary\":\"national district-court candidate; district allocation and case-type effects require separate review\",\"permanent_judgeships\":65,\"temporary_judgeships\":1}},\"intervention\":{{\"mechanism\":\"phase in additional federal district judgeships every two years through 2035\",\"implementing_owner\":\"Congress President Senate AOUSC and affected district courts\",\"eligibility_rule\":\"judgeship allocations specified by legislation; no individual case rule\",\"exclusions\":\"no docket assignment adjudication detention prosecution or sentencing recommendation\",\"existing_treatment_or_programmed_work\":\"current authorized judgeships senior judges and magistrate judges remain the null capacity\"}},\"outcomes\":{{\"bounded_marginal_effect_or_null\":null,\"effect_population\":\"civil cases and criminal defendants in affected districts\",\"horizon\":\"phased FY2025-FY2035\",\"uncertainty\":\"CBO scores cost but not delay resolution rights safety or distribution effects\",\"transferability_boundary\":\"federal district courts only\"}},\"service_floors\":{{\"access\":\"filing-to-disposition and aged-pending policy values absent\",\"quality_safety\":\"evidence integrity victim safety and durable resolution values absent\",\"equity_distribution\":\"district case-type and demographic incidence absent\",\"adequacy_resilience\":\"vacancy senior-judge magistrate and stress capacity not reconciled\",\"delivery_feasibility\":\"confirmation staffing facilities security and technology milestones absent\",\"counsel_liberty_victim_notice_pass\":null}},\"costs\":{{\"price_year_or_null\":\"nominal federal fiscal years\",\"gross_cost_or_null\":{},\"implementation_cost_or_null\":null,\"maintenance_cost_or_null\":null,\"offsets_or_null\":0,\"dedicated_receipts_or_null\":null,\"state_local_private_shift_or_null\":null,\"net_cost_or_null\":{},\"public_savings\":null}},\"fiscal_bridge\":{{\"gross_public_funding_need_or_null\":{},\"delivery_efficiency_public_savings_or_null\":null,\"external_economic_benefit_or_null\":null,\"operator_or_private_revenue_or_null\":null,\"legally_dedicated_public_receipts_or_null\":null,\"collection_and_financing_cost_or_null\":null,\"net_public_fiscal_pressure_or_null\":{},\"revenue_authority\":\"none\",\"demand_and_incidence_basis\":\"CBO estimate assuming enactment and appropriation\",\"netting_rule\":\"direct judge compensation and appropriated court operations are added once; no delay or detention savings scored\"}},\"adaptive_pathways\":{{\"pathway_classes\":\"phased court-capacity investment\",\"peer_goal_basis\":null,\"evaluation_horizons\":\"biennial additions through 2035 and annual caseflow review\",\"realization_owner_or_null\":\"AOUSC and affected district courts after enactment appointment confirmation and appropriation\",\"transition_and_implementation_cost_or_null\":null,\"uncertainty_and_downside\":\"appointments appropriations district allocation and causal caseflow effects uncertain\",\"service_floor_and_distribution_result\":\"held\",\"overlap_and_non_additivity\":\"court cost does not include prosecution defense detention corrections or litigant costs\",\"observation_cadence\":\"annual AOUSC caseflow and biennial judgeship review\",\"reopen_triggers\":\"enactment appropriation staffed seats and district-level outcome/floor evidence\",\"current_disposition\":\"held\"}},\"delivery\":{{\"capacity\":\"65 permanent and one temporary judgeship authorized by candidate\",\"schedule\":\"additional judgeships every two years from 2025 through 2035\",\"milestones\":null,\"useful_life\":\"life tenure for permanent Article III judgeships\",\"sunset_or_review\":\"temporary judgeship and biennial reporting provisions require separate review\"}},\"overlap\":{{\"shared_projects\":\"court staff facilities security technology and detention-space reporting\",\"shared_cost_allocation\":\"CBO direct and appropriated outlays retained separately\",\"other_lane_interactions\":\"JUS DIS ISF\",\"non_additivity_rule\":\"do not infer or add prosecution defense detention corrections litigant or social savings\"}},\"readiness\":{{\"domain_evidence_ready\":true,\"candidate_bounded\":true,\"outcome_ready\":false,\"cost_ready\":true,\"floors_ready\":false,\"delivery_ready\":false,\"overlap_ready\":false,\"taxlane_admission_ready\":false}},\"claim_boundaries\":{{\"domain_finding_allowed\":true,\"candidate_recommendation_allowed\":false,\"individual_decision_allowed\":false,\"savings_allowed\":false,\"allocation_allowed\":false,\"rate_change_allowed\":false,\"public_release_allowed\":false}}}}",
        result.combined_outlays_millions,
        result.combined_outlays_millions,
        result.combined_outlays_millions,
        result.combined_outlays_millions
    )
}

pub fn run(command: &str, input: &str) -> Result<String, String> {
    if !input.contains("# evidence_label=official_legislative_candidate") {
        return Err("candidate command requires official_legislative_candidate evidence".into());
    }
    if !input.contains("# legislative_status=ordered_reported_house_committee_not_enacted") {
        return Err("candidate fixture must state legislative status".into());
    }
    let result = analyze(&parse(input)?);
    match command {
        "candidate-baseline" => Ok(baseline_json(&result)),
        "candidate-held-pack" => Ok(held_pack_json(&result)),
        _ => Err(format!("unknown candidate command: {command}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE: &str =
        include_str!("../../../fixtures/official/cbo-hr1702-judges-act-2025-cost-estimate.tsv");

    #[test]
    fn reconciles_direct_and_appropriated_costs() {
        let result = analyze(&parse(FIXTURE).unwrap());
        assert_eq!(result.direct_outlays_millions, 111);
        assert_eq!(result.appropriation_outlays_millions, 283);
        assert_eq!(result.combined_outlays_millions, 394);
        assert_eq!(result.first_full_scored_year_millions, 8);
        assert_eq!(result.end_year_millions, 71);
    }

    #[test]
    fn capacity_is_not_promoted_to_outcome_or_savings() {
        let output = baseline_json(&analyze(&parse(FIXTURE).unwrap()));
        assert!(output.contains("\"capacity_is_outcome\":false"));
        assert!(output.contains("\"cost_is_savings\":false"));
    }

    #[test]
    fn held_pack_is_bounded_and_cost_ready_only() {
        let pack = held_pack_json(&analyze(&parse(FIXTURE).unwrap()));
        assert!(pack.contains("\"candidate_bounded\":true"));
        assert!(pack.contains("\"cost_ready\":true"));
        assert!(pack.contains("\"outcome_ready\":false"));
        assert!(pack.contains("\"floors_ready\":false"));
        assert!(pack.contains("\"taxlane_admission_ready\":false"));
        assert!(pack.contains("\"public_savings\":null"));
    }

    #[test]
    fn rejects_missing_fiscal_year() {
        let changed = FIXTURE.replace("2030\t10\t26\n", "");
        assert!(parse(&changed).is_err());
    }

    #[test]
    fn rejects_negative_costs() {
        let changed = FIXTURE.replacen("2026\t3\t5", "2026\t-3\t5", 1);
        assert!(parse(&changed).is_err());
    }
}

use crate::domain::money::ExactMoney;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CalculationStatus {
    Available,
    Provisional,
    Unavailable,
}

impl CalculationStatus {
    pub const fn code(self) -> &'static str {
        match self {
            Self::Available => "disponible",
            Self::Provisional => "provisional",
            Self::Unavailable => "no_disponible",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CalculationReason {
    PendingGrossYieldFormula,
    PendingNetYieldTreatment,
    PendingNetYield,
    ApprovedDemoPv01,
    ApprovedDemoPv02,
    ApprovedDemoPv03,
}

impl CalculationReason {
    pub const fn code(self) -> &'static str {
        match self {
            Self::PendingGrossYieldFormula => "pending_gross_yield_formula",
            Self::PendingNetYieldTreatment => "pending_net_yield_treatment",
            Self::PendingNetYield => "pending_net_yield",
            Self::ApprovedDemoPv01 => "approved_demo_pv01",
            Self::ApprovedDemoPv02 => "approved_demo_pv02",
            Self::ApprovedDemoPv03 => "approved_demo_pv03",
        }
    }
}

#[derive(Debug, Clone)]
pub enum FinancialResult<T> {
    Available(T),
    Provisional { value: T, reason: CalculationReason },
    Unavailable { reason: CalculationReason },
}

impl<T> FinancialResult<T> {
    pub fn status(&self) -> CalculationStatus {
        match self {
            Self::Available(_) => CalculationStatus::Available,
            Self::Provisional { .. } => CalculationStatus::Provisional,
            Self::Unavailable { .. } => CalculationStatus::Unavailable,
        }
    }

    pub fn reason(&self) -> Option<CalculationReason> {
        match self {
            Self::Available(_) => None,
            Self::Provisional { reason, .. } | Self::Unavailable { reason } => Some(*reason),
        }
    }

    pub fn value(&self) -> Option<&T> {
        match self {
            Self::Available(value) => Some(value),
            Self::Provisional { value, .. } => Some(value),
            Self::Unavailable { .. } => None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Coverage {
    Covered,
    Deficit { amount: ExactMoney },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn estados_y_motivos_exponen_codigo() {
        assert_eq!(CalculationStatus::Available.code(), "disponible");
        assert_eq!(CalculationStatus::Provisional.code(), "provisional");
        assert_eq!(CalculationStatus::Unavailable.code(), "no_disponible");

        assert_eq!(
            CalculationReason::PendingGrossYieldFormula.code(),
            "pending_gross_yield_formula"
        );
        assert_eq!(
            CalculationReason::ApprovedDemoPv03.code(),
            "approved_demo_pv03"
        );
    }

    #[test]
    fn resultado_tipado_expone_estado_motivo_y_valor() {
        let available = FinancialResult::Available(10);
        let provisional = FinancialResult::Provisional {
            value: 20,
            reason: CalculationReason::ApprovedDemoPv01,
        };
        let unavailable: FinancialResult<i32> = FinancialResult::Unavailable {
            reason: CalculationReason::PendingNetYield,
        };

        assert_eq!(available.status(), CalculationStatus::Available);
        assert_eq!(available.reason(), None);
        assert_eq!(available.value(), Some(&10));

        assert_eq!(provisional.status(), CalculationStatus::Provisional);
        assert_eq!(
            provisional.reason(),
            Some(CalculationReason::ApprovedDemoPv01)
        );
        assert_eq!(provisional.value(), Some(&20));

        assert_eq!(unavailable.status(), CalculationStatus::Unavailable);
        assert_eq!(
            unavailable.reason(),
            Some(CalculationReason::PendingNetYield)
        );
        assert_eq!(unavailable.value(), None);
    }
}

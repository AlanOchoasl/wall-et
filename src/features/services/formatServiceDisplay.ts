import type { ServiceStatusDto } from "../../shared/api";

export {
  formatExactMinor,
  formatExactMoney,
  formatMinorUnits,
  formatMoney,
} from "../../shared/moneyDisplay";

export function formatNotes(notes: string | null): string {
  return notes ?? "—";
}

export function formatServiceStatus(status: ServiceStatusDto): string {
  return status === "active" ? "Activo" : "Congelado";
}

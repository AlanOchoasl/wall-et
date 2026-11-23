import { invokeCommand } from "./commands";

export type CurrencyCode = "ARS" | "USD";

export type ServiceStatusDto = "active" | "frozen";

export type MoneyDto = {
  minorUnits: string;
  currencyCode: CurrencyCode;
};

export type ExactMoneyDto = {
  numeratorMinor: string;
  denominator: string;
  currencyCode: CurrencyCode;
};

export type ServiceRecordDto = {
  id: string;
  name: string;
  totalCost: MoneyDto;
  myShare: ExactMoneyDto;
  sharedPeopleCount: number;
  notes: string | null;
  status: ServiceStatusDto;
  createdAt: string;
  updatedAt: string;
};

export type CreateServiceInputDto = {
  name: string;
  totalCost: string;
  currencyCode: CurrencyCode;
  sharedPeopleCount: number;
  notes?: string | null;
};

export type UpdateServiceInputDto = {
  name: string;
  totalCost: string;
  sharedPeopleCount: number;
  notes?: string | null;
};

export type DeleteServiceResultDto = {
  outcome: "deleted" | "cancelled";
  id: string;
};

export type ServiceMutationResultDto = {
  outcome: "frozen" | "already_frozen" | "reactivated" | "already_active";
};

export type CurrencyServiceTotalsDto = {
  totalCost: MoneyDto;
  userShare: ExactMoneyDto;
};

export type ServiceTotalsDto = {
  ars: CurrencyServiceTotalsDto;
  usd: CurrencyServiceTotalsDto;
};

export type AppStatus = {
  appName: string;
  version: string;
  databaseReady: boolean;
  schemaVersion: number;
  currencyCode: string;
};

export function getAppStatus(): Promise<AppStatus> {
  return invokeCommand<AppStatus>("get_app_status");
}

export function createService(
  input: CreateServiceInputDto,
): Promise<ServiceRecordDto> {
  return invokeCommand<ServiceRecordDto>("create_service", { input });
}

export function updateService(
  id: string,
  input: UpdateServiceInputDto,
): Promise<ServiceRecordDto> {
  return invokeCommand<ServiceRecordDto>("update_service", { id, input });
}

export function deleteService(
  id: string,
  confirmed: boolean,
): Promise<DeleteServiceResultDto> {
  return invokeCommand<DeleteServiceResultDto>("delete_service", {
    id,
    confirmed,
  });
}

export function freezeService(id: string): Promise<ServiceMutationResultDto> {
  return invokeCommand<ServiceMutationResultDto>("freeze_service", { id });
}

export function reactivateService(
  id: string,
): Promise<ServiceMutationResultDto> {
  return invokeCommand<ServiceMutationResultDto>("reactivate_service", { id });
}

export function listFrozenServices(): Promise<ServiceRecordDto[]> {
  return invokeCommand<ServiceRecordDto[]>("list_frozen_services");
}

export function listActiveServices(): Promise<ServiceRecordDto[]> {
  return invokeCommand<ServiceRecordDto[]>("list_active_services");
}

export function getServiceTotals(): Promise<ServiceTotalsDto> {
  return invokeCommand<ServiceTotalsDto>("get_service_totals");
}

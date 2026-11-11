// The organisation at a glance: members by standing, this year's giving, grants awaiting or awarded,
// and volunteers' approved hours, each counted or added up over every record, on the workspace's
// year. Each figure opens its list. Before the books are named, it says to name them first.

import { useQuery } from "@tanstack/react-query";
import { Link } from "react-router";
import { api, docPath, todayIso, t, isolate } from "@ferrum/client";
import { Settings } from "lucide-react";
import { ButtonLink, HomeSkeleton, LoadFailed, PageHead, StatePanel, Tile, TileGrid, useShellTitle } from "@ferrum/kit";

interface Row {
  [k: string]: unknown;
}

interface Page {
  rows: Row[];
  next_cursor?: string | null;
  total?: number;
}

const page = (entity: string, filters: unknown[], extra: Record<string, string | number | boolean | undefined>) =>
  api.get<Page>(docPath(entity), { filters: JSON.stringify(filters), ...extra });

/** Every matching row, page after page: a figure covers all of them, not one page's 500. Cursors
 * need the creation order. */
export async function every(entity: string, filters: unknown[], fields: string, get = page): Promise<Row[]> {
  const out: Row[] = [];
  let cursor: string | undefined;
  do {
    const p = await get(entity, filters, { fields, limit: 500, cursor, sort: "created_at desc" });
    out.push(...p.rows);
    cursor = p.next_cursor ?? undefined;
  } while (cursor);
  return out;
}

const count = (entity: string, filters: unknown[]) => page(entity, filters, { fields: "id", limit: 1, count: true }).then((p) => p.total ?? p.rows.length);

/** Sum of a field, as the list returns it (decimals come as strings). */
export const sum = (list: Row[], field: string) => list.reduce((acc, r) => acc + Number(r[field] ?? 0), 0);

export function Home() {
  useShellTitle(t("Nonprofit"));
  const year = todayIso().slice(0, 4);
  const q = useQuery({
    queryKey: ["nonprofit", "home", year],
    queryFn: async () => {
      const [settings, active, lapsed, unpaid, donations, grants, volunteers] = await Promise.all([
        page("NonprofitSettings", [], { fields: "id", limit: 1 }),
        count("Member", [["status", "eq", "Active"]]),
        count("Member", [["status", "eq", "Lapsed"]]),
        count("Member", [["status", "eq", "Applicant"]]),
        every("Donation", [["docstatus", "eq", 1], ["date", "gte", `${year}-01-01`], ["date", "lte", `${year}-12-31`]], "amount"),
        count("Grant", [["status", "in", ["Submitted", "Awarded"]]]),
        every("Volunteer", [["active", "eq", true]], "hours"),
      ]);
      return { named: settings.rows.length > 0, active, lapsed, unpaid, donations, grants, volunteers };
    },
  });
  const head = <PageHead title={t("Nonprofit")} />;
  if (q.isPending) return <>{head}<HomeSkeleton /></>;
  if (q.isError) return <>{head}<LoadFailed error={q.error} onRetry={() => void q.refetch()} /></>;
  const { named, active, lapsed, unpaid, donations, grants, volunteers } = q.data;
  if (!named)
    return (
      <>
        {head}
        <StatePanel kind="empty" icon={Settings} title={t("Name the books first")} action={<ButtonLink to="/e/NonprofitSettings" variant="primary">{t("Nonprofit Settings")}</ButtonLink>}>
          {t("Donations post to the books and memberships are invoiced from a company: set the company, the account donations are received into and the income they are credited to in")} <Link to="/e/NonprofitSettings">{t("Nonprofit Settings")}</Link>.
        </StatePanel>
      </>
    );
  const figures: { label: string; value: string; to: string }[] = [
    { label: t("Active members"), value: String(active), to: "/e/Member" },
    { label: t("Members not yet paid"), value: String(unpaid), to: "/e/Member" },
    { label: t("Lapsed members"), value: String(lapsed), to: "/e/Member" },
    { label: t("Given in {year}", { year }), value: sum(donations, "amount").toLocaleString(undefined, { maximumFractionDigits: 2 }), to: "/e/Donation" },
    { label: t("Grants awaiting or awarded"), value: String(grants), to: "/e/Grant" },
    { label: t("Volunteers' approved hours"), value: sum(volunteers, "hours").toLocaleString(), to: "/e/Volunteer" },
  ];
  return (
    <>
      {head}
      <TileGrid label={t("Nonprofit")} className="nf-figures">
        {figures.map((f) => (
          <Tile key={f.label} to={f.to} title={isolate(f.label)} meta={<span className="mt-1 text-2xl font-semibold tabular-nums text-foreground">{isolate(f.value)}</span>} />
        ))}
      </TileGrid>
    </>
  );
}

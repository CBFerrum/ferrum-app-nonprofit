// The organisation at a glance: members by standing, this year's giving, grants awaiting or awarded,
// and volunteers' approved hours, each counted or added up over every record, on the workspace's
// year. Each figure opens its list. Before the books are named, it says to name them first.

import { useQuery } from "@tanstack/react-query";
import { Link } from "react-router";
import { api, docPath, todayIso } from "@ferrum/client";
import { EmptyState, ErrorState, PageLoading, useShellTitle } from "@ferrum/kit";

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
  useShellTitle("Nonprofit");
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
  if (q.isPending) return <PageLoading />;
  if (q.isError) return <ErrorState error={q.error} onRetry={() => void q.refetch()} />;
  const { named, active, lapsed, unpaid, donations, grants, volunteers } = q.data;
  if (!named)
    return (
      <div className="np-page">
        <EmptyState title="Name the books first" icon="settings">
          <p className="muted">
            Donations post to the books and memberships are invoiced from a company: set the company, the account donations are received into and the
            income they are credited to in <Link to="/e/NonprofitSettings">Nonprofit Settings</Link>.
          </p>
        </EmptyState>
      </div>
    );
  const figures: { label: string; value: string; to: string }[] = [
    { label: "Active members", value: String(active), to: "/e/Member" },
    { label: "Members not yet paid", value: String(unpaid), to: "/e/Member" },
    { label: "Lapsed members", value: String(lapsed), to: "/e/Member" },
    { label: `Given in ${year}`, value: sum(donations, "amount").toLocaleString(undefined, { maximumFractionDigits: 2 }), to: "/e/Donation" },
    { label: "Grants awaiting or awarded", value: String(grants), to: "/e/Grant" },
    { label: "Volunteers' approved hours", value: sum(volunteers, "hours").toLocaleString(), to: "/e/Volunteer" },
  ];
  return (
    <div className="np-page">
      <div className="np-columns">
        {figures.map((f) => (
          <Link key={f.label} to={f.to} className="card card-pad stack">
            <span className="muted">{f.label}</span>
            <strong style={{ fontSize: 28 }}>{f.value}</strong>
          </Link>
        ))}
      </div>
    </div>
  );
}

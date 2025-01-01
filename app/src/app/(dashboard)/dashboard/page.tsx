import { withAuth } from "@/components/protected-page/with-auth";

function Dashboard() {
  return <div>Teste</div>;
}

export default withAuth(Dashboard);

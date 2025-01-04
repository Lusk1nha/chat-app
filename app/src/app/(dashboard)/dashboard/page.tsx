import { withAuth } from "@/components/protected-page/with-auth";
import profileService from "@/shared/factories/profile-factory";
import { unstable_cache } from "next/cache";

const getCurrentProfile = unstable_cache(
  async () => {
    return await profileService.getCurrentProfile();
  },
  ["profile"],
  { revalidate: 3600, tags: ["profile"] }
);

async function Dashboard() {
  const profile = await getCurrentProfile();
  console.log(profile);

  return <div>teste</div>;
}

export default withAuth(Dashboard);

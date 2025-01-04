import Spinner from "@/components/ui/spinner";

export default function LoadingPage() {
  return (
    <div className="bg-background w-full h-screen flex items-center justify-center">
      <div className="max-w-[520px] w-full h-full flex flex-col items-center justify-center gap-y-2 px-4">
        <div className="h-20 flex items-center justify-center">
          <Spinner className="w-20 h-20 text-primary" />
        </div>

        <div className="flex flex-col items-center justify-center">
          <h4 className="text-xl font-bold">Loading...</h4>
          <p className="text-sm text-center">
            Please wait while we load the page.
          </p>
        </div>
      </div>
    </div>
  );
}

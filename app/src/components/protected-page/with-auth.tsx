import ProtectedComponent from "@/components/protected-page/protected-page";

export function withAuth<T extends object>(
  WrappedComponent: React.ComponentType<T>
) {
  const ProtectedHOC = (props: T) => {
    return (
      <ProtectedComponent>
        <WrappedComponent {...props} />
      </ProtectedComponent>
    );
  };

  return ProtectedHOC;
}

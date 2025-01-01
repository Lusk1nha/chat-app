import PublicComponent from "@/components/public-page/public-page";

export function withPublic<T extends object>(
  WrappedComponent: React.ComponentType<T>
) {
  const PublicHOC = (props: T) => {
    return (
      <PublicComponent>
        <WrappedComponent {...props} />
      </PublicComponent>
    );
  };

  return PublicHOC;
}

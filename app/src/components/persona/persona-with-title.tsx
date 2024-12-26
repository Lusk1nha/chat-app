import { IPersonaProps, Persona } from "./persona";

interface IPersonaWithTitleProps extends IPersonaProps {
  title: string;
  aux?: string;
}

export function PersonaWithTitle(props: Readonly<IPersonaWithTitleProps>) {
  const { title, aux, ...persona } = props;

  return (
    <div className="flex items-center gap-x-3">
      <Persona {...persona} />

      <div className="flex flex-col">
        <h2 className="text-sm font-semibold">{title}</h2>
        <p className="text-xs font-medium text-gray-500">{aux}</p>
      </div>
    </div>
  );
}

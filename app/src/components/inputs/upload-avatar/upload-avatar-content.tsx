import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";
import { AnimatePresence, motion } from "framer-motion";

interface IUploadAvatarContentProps {
  value: File;
}

export function UploadAvatarContent(
  props: Readonly<IUploadAvatarContentProps>
) {
  const { value } = props;

  return (
    <AnimatePresence>
      <motion.div
        className="w-full border-2 border-dashed rounded-lg flex flex-col gap-1 p-6 items-center border-accent transition-all"
        initial={{ opacity: 0 }}
        animate={{ opacity: 1 }}
        exit={{ opacity: 0 }}
      >
        <Avatar className="w-16 md:w-24 h-16 md:h-24 border-2 border-dashed border-primary p-[0.1rem]">
          <AvatarImage
            className="rounded-full"
            src={URL.createObjectURL(value)}
            alt={value.name}
          />
          <AvatarFallback>{value.name}</AvatarFallback>
        </Avatar>
      </motion.div>
    </AnimatePresence>
  );
}

"use client";

import { FormInput } from "@/components/inputs/form-input";
import { FormTextarea } from "@/components/inputs/form-textarea";
import FormUploadAvatar from "@/components/inputs/form-upload-avatar";

import { Logo } from "@/components/logo";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle
} from "@/components/ui/card";
import { Form } from "@/components/ui/form";
import { Separator } from "@/components/ui/separator";
import {
  WizardValidation,
  wizardValidation
} from "@/shared/validations/wizard-validation";
import { zodResolver } from "@hookform/resolvers/zod";
import { Save } from "lucide-react";
import { useForm } from "react-hook-form";

export default function WizardPage() {
  const form = useForm<WizardValidation>({
    resolver: zodResolver(wizardValidation),
    defaultValues: {
      name: "",
      bio: "",
      avatar_url: null
    }
  });

  async function handleSaveProfile(data: WizardValidation) {
    const response = wizardValidation.parse(data);
    console.log(response);
  }

  return (
    <div className="bg-background w-full h-screen flex items-center justify-center px-4">
      <div className="container h-full flex max-w-2xl flex-col items-center justify-center gap-4">
        <div className="flex flex-col pt-10">
          <h1 className="text-3xl text-center items-center">Profile editor</h1>
          <p className="text-center text-base text-muted-foreground">
            Let's get started by creating your profile
          </p>
          <p className="mt-1 text-center text-sm text-muted-foreground">
            This settings will be used to customize your profile and provide a
            better experience. You can always change these settings later.
          </p>
        </div>

        <Separator />

        <div className="w-full flex flex-col gap-y-4">
          <Card className="w-full h-full">
            <CardHeader>
              <CardTitle>Initial setup</CardTitle>
              <CardDescription>
                We will guide you through the process of creating your profile
              </CardDescription>
            </CardHeader>

            <CardContent className="w-full h-full">
              <Form {...form}>
                <form
                  onSubmit={form.handleSubmit(handleSaveProfile)}
                  className="w-full flex flex-col gap-y-4"
                >
                  <FormInput
                    name="name"
                    label="Username"
                    control={form.control}
                    field={{
                      placeholder: "Insert your username"
                    }}
                    required
                  />

                  <FormTextarea
                    name="bio"
                    label="Bio"
                    control={form.control}
                    field={{
                      placeholder: "Insert your username",
                      rows: 4
                    }}
                    required
                  />

                  <FormUploadAvatar
                    name="avatar_url"
                    label="Profile picture"
                    control={form.control}
                    required
                  />

                  <Button type="submit">
                    <Save /> Save profile
                  </Button>
                </form>
              </Form>
            </CardContent>
          </Card>

          <div className="w-full flex justify-center">
            <Logo />
          </div>
        </div>
      </div>
    </div>
  );
}

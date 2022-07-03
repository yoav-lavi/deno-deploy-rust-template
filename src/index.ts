import { serve } from "https://deno.land/std@0.142.0/http/server.ts";
import { handler } from "../build/{{project_name}}.js";

serve(handler);

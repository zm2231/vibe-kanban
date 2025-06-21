import { useState, useMemo } from "react";
import { Card, CardContent } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import { Chip } from "@/components/ui/chip";
import { FileText, MessageSquare } from "lucide-react";
import { ConversationViewer } from "./ConversationViewer";
import type { ExecutionProcess } from "shared/types";

interface ExecutionOutputViewerProps {
  executionProcess: ExecutionProcess;
  executor?: string;
}

export function ExecutionOutputViewer({
  executionProcess,
  executor,
}: ExecutionOutputViewerProps) {
  const [viewMode, setViewMode] = useState<"conversation" | "raw">(
    executor === "amp" ? "conversation" : "raw"
  );

  const isAmpExecutor = executor === "amp";
  const hasStdout = !!executionProcess.stdout;
  const hasStderr = !!executionProcess.stderr;

  // Check if stdout looks like JSONL (for Amp executor)
  const isValidJsonl = useMemo(() => {
    if (!isAmpExecutor || !executionProcess.stdout) return false;

    try {
      const lines = executionProcess.stdout
        .split("\n")
        .filter((line) => line.trim());
      if (lines.length === 0) return false;

      // Try to parse at least the first few lines as JSON
      const testLines = lines.slice(0, Math.min(3, lines.length));
      return testLines.every((line) => {
        try {
          JSON.parse(line);
          return true;
        } catch {
          return false;
        }
      });
    } catch {
      return false;
    }
  }, [isAmpExecutor, executionProcess.stdout]);

  if (!hasStdout && !hasStderr) {
    return (
      <Card className="bg-muted border-none">
        <CardContent className="p-3">
          <div className="text-xs text-muted-foreground italic text-center">
            Waiting for output...
          </div>
        </CardContent>
      </Card>
    );
  }

  return (
    <Card className="">
      <CardContent className="p-3">
        <div className="space-y-3">
          {/* View mode toggle for Amp executor with valid JSONL */}
          {isAmpExecutor && isValidJsonl && hasStdout && (
            <div className="flex items-center justify-between">
              <div className="flex items-center gap-2">
                <Badge variant="outline" className="text-xs">
                  {executor} output
                </Badge>
              </div>
              <div className="flex items-center gap-1">
                <Button
                  variant={viewMode === "conversation" ? "default" : "ghost"}
                  size="sm"
                  onClick={() => setViewMode("conversation")}
                  className="h-7 px-2 text-xs"
                >
                  <MessageSquare className="h-3 w-3 mr-1" />
                  Conversation
                </Button>
                <Button
                  variant={viewMode === "raw" ? "default" : "ghost"}
                  size="sm"
                  onClick={() => setViewMode("raw")}
                  className="h-7 px-2 text-xs"
                >
                  <FileText className="h-3 w-3 mr-1" />
                  Raw
                </Button>
              </div>
            </div>
          )}

          {/* Output content */}
          {hasStdout && (
            <div>
              {isAmpExecutor && isValidJsonl && viewMode === "conversation" ? (
                <ConversationViewer
                  jsonlOutput={executionProcess.stdout || ""}
                />
              ) : (
                <div>
                  <pre className="text-xs overflow-x-auto whitespace-pre-wrap p-2">
                    {executionProcess.stdout}
                  </pre>
                </div>
              )}
            </div>
          )}

          {hasStderr && (
            <div>
              <pre className="text-xs overflow-x-auto whitespace-pre-wrap p-2 text-red-600">
                {executionProcess.stderr}
              </pre>
            </div>
          )}
        </div>
      </CardContent>
    </Card>
  );
}

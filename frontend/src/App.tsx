import { useState, useEffect } from "react";
import { BrowserRouter, Routes, Route } from "react-router-dom";
import { Navbar } from "@/components/layout/navbar";
import { Projects } from "@/pages/projects";
import { ProjectTasks } from "@/pages/project-tasks";
import { TaskDetailsPage } from "@/pages/task-details";
import { TaskAttemptComparePage } from "@/pages/task-attempt-compare";
import { Settings } from "@/pages/Settings";
import { DisclaimerDialog } from "@/components/DisclaimerDialog";
import { ConfigProvider, useConfig } from "@/components/config-provider";
import type { Config, ApiResponse } from "shared/types";

function AppContent() {
  const { config, updateConfig, loading } = useConfig();
  const [showDisclaimer, setShowDisclaimer] = useState(false);
  const showNavbar = true;

  useEffect(() => {
    if (config) {
      setShowDisclaimer(!config.disclaimer_acknowledged);
    }
  }, [config]);

  const handleDisclaimerAccept = async () => {
    if (!config) return;

    updateConfig({ disclaimer_acknowledged: true });
    
    try {
      const response = await fetch("/api/config", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ ...config, disclaimer_acknowledged: true }),
      });

      const data: ApiResponse<Config> = await response.json();

      if (data.success) {
        setShowDisclaimer(false);
      }
    } catch (err) {
      console.error("Error saving config:", err);
    }
  };

  if (loading) {
    return (
      <div className="min-h-screen bg-background flex items-center justify-center">
        <div className="text-center">
          <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-primary mx-auto"></div>
          <p className="mt-2 text-muted-foreground">Loading...</p>
        </div>
      </div>
    );
  }

  return (
    <div className="h-screen flex flex-col bg-background">
      <DisclaimerDialog
        open={showDisclaimer}
        onAccept={handleDisclaimerAccept}
      />
      {showNavbar && <Navbar />}
      <div className="flex-1 overflow-y-scroll">
        <Routes>
          <Route path="/" element={<Projects />} />
          <Route path="/projects" element={<Projects />} />
          <Route path="/projects/:projectId" element={<Projects />} />
          <Route path="/projects/:projectId/tasks" element={<ProjectTasks />} />
          <Route
            path="/projects/:projectId/tasks/:taskId"
            element={<TaskDetailsPage />}
          />
          <Route
            path="/projects/:projectId/tasks/:taskId/attempts/:attemptId/compare"
            element={<TaskAttemptComparePage />}
          />
          <Route path="/settings" element={<Settings />} />
        </Routes>
      </div>
    </div>
  );
}

function App() {
  return (
    <BrowserRouter>
      <ConfigProvider>
        <AppContent />
      </ConfigProvider>
    </BrowserRouter>
  );
}

export default App;

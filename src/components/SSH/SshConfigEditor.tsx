import { useState, useEffect, useCallback, useRef } from "react";
import {
  X,
  Save,
  AlertTriangle,
  CheckCircle,
  FileText,
  Loader2,
  FolderOpen,
  File,
  ChevronRight,
} from "lucide-react";
import { clsx } from "clsx";
import {
  readSshConfig,
  saveSshConfig,
  validateSshConfig,
  listSshConfigFiles,
  type SshConfigFile,
} from "../../services/tauri";

interface SshConfigEditorProps {
  open: boolean;
  onClose: () => void;
  onSaved: () => void;
}

interface FileTab {
  file: SshConfigFile;
  content: string;
  original: string;
  dirty: boolean;
}

export function SshConfigEditor({ open, onClose, onSaved }: SshConfigEditorProps) {
  const [configFiles, setConfigFiles] = useState<SshConfigFile[]>([]);
  const [tabs, setTabs] = useState<Map<string, FileTab>>(new Map());
  const [activeFilePath, setActiveFilePath] = useState<string>("");

  const [loading, setLoading] = useState(false);
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [warnings, setWarnings] = useState<string[]>([]);
  const [saveSuccess, setSaveSuccess] = useState(false);
  const [validateSuccess, setValidateSuccess] = useState(false);

  const textareaRef = useRef<HTMLTextAreaElement>(null);
  const lineNumbersRef = useRef<HTMLDivElement>(null);

  const activeTab = tabs.get(activeFilePath);
  const content = activeTab?.content ?? "";
  const isDirty = activeTab?.dirty ?? false;
  const anyDirty = [...tabs.values()].some((t) => t.dirty);

  // Load file list when opened
  useEffect(() => {
    if (!open) return;
    setLoading(true);
    setError(null);
    setWarnings([]);
    setSaveSuccess(false);
    setValidateSuccess(false);
    setTabs(new Map());
    setActiveFilePath("");

    listSshConfigFiles()
      .then(async (files) => {
        setConfigFiles(files);
        // Load main config by default
        if (files.length > 0) {
          const mainFile = files[0];
          await loadFile(mainFile);
        }
      })
      .catch((e) => setError(String(e)))
      .finally(() => setLoading(false));
  }, [open]);

  // Sync line numbers scroll with textarea
  useEffect(() => {
    const textarea = textareaRef.current;
    const lineNumbers = lineNumbersRef.current;
    if (!textarea || !lineNumbers) return;
    const onScroll = () => {
      lineNumbers.scrollTop = textarea.scrollTop;
    };
    textarea.addEventListener("scroll", onScroll);
    return () => textarea.removeEventListener("scroll", onScroll);
  }, [activeFilePath]);

  const loadFile = useCallback(async (file: SshConfigFile) => {
    setError(null);
    setWarnings([]);
    setSaveSuccess(false);
    setValidateSuccess(false);
    try {
      const text = await readSshConfig(file.path);
      setTabs((prev) => {
        const next = new Map(prev);
        // Don't reload if already loaded and dirty
        if (next.has(file.path) && next.get(file.path)!.dirty) {
          return next;
        }
        next.set(file.path, {
          file,
          content: text,
          original: text,
          dirty: false,
        });
        return next;
      });
      setActiveFilePath(file.path);
    } catch (e) {
      setError(String(e));
    }
  }, []);

  const selectFile = useCallback(async (file: SshConfigFile) => {
    if (tabs.has(file.path)) {
      setActiveFilePath(file.path);
      setError(null);
      setWarnings([]);
      setSaveSuccess(false);
      setValidateSuccess(false);
    } else {
      await loadFile(file);
    }
  }, [tabs, loadFile]);

  const updateContent = useCallback((newContent: string) => {
    setTabs((prev) => {
      const next = new Map(prev);
      const tab = next.get(activeFilePath);
      if (tab) {
        next.set(activeFilePath, {
          ...tab,
          content: newContent,
          dirty: newContent !== tab.original,
        });
      }
      return next;
    });
  }, [activeFilePath]);

  // Keyboard shortcut: Cmd/Ctrl+S to save
  useEffect(() => {
    if (!open) return;
    const handler = (e: KeyboardEvent) => {
      if ((e.metaKey || e.ctrlKey) && e.key === "s") {
        e.preventDefault();
        handleSave();
      }
    };
    window.addEventListener("keydown", handler);
    return () => window.removeEventListener("keydown", handler);
  }, [open, activeFilePath, tabs, saving]);

  const handleValidate = useCallback(async () => {
    setError(null);
    setWarnings([]);
    setValidateSuccess(false);
    try {
      const w = await validateSshConfig(content);
      setWarnings(w);
      if (w.length === 0) {
        setValidateSuccess(true);
        setTimeout(() => setValidateSuccess(false), 5000);
      }
      return w.length === 0;
    } catch (e) {
      setError(String(e));
      return false;
    }
  }, [content]);

  const handleSave = useCallback(async () => {
    if (saving || !isDirty) return;
    setSaving(true);
    setError(null);
    setWarnings([]);
    setSaveSuccess(false);
    try {
      const w = await saveSshConfig(content, activeFilePath || undefined);
      setWarnings(w);
      // Mark saved
      setTabs((prev) => {
        const next = new Map(prev);
        const tab = next.get(activeFilePath);
        if (tab) {
          next.set(activeFilePath, { ...tab, original: content, dirty: false });
        }
        return next;
      });
      setSaveSuccess(true);
      onSaved();
      setTimeout(() => setSaveSuccess(false), 3000);
    } catch (e) {
      setError(String(e));
    } finally {
      setSaving(false);
    }
  }, [content, activeFilePath, saving, isDirty, onSaved]);

  const handleClose = () => {
    if (anyDirty) {
      if (!window.confirm("有未保存的修改，确认关闭？")) return;
    }
    onClose();
  };

  if (!open) return null;

  const activeFileName = activeTab?.file.name ?? "";

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm">
      <div className="bg-surface border border-border rounded-2xl shadow-2xl flex flex-col w-[92vw] max-w-5xl h-[88vh]">
        {/* Header */}
        <div className="flex items-center justify-between px-5 py-3 border-b border-border shrink-0">
          <div className="flex items-center gap-2">
            <FileText className="w-4.5 h-4.5 text-primary" />
            <h2 className="text-sm font-semibold">编辑 SSH 配置</h2>
            {activeFileName && (
              <>
                <ChevronRight className="w-3 h-3 text-text-dim" />
                <span className="text-xs text-text-dim font-mono">{activeFileName}</span>
              </>
            )}
            {isDirty && (
              <span className="text-[10px] px-1.5 py-0.5 rounded bg-warning/15 text-warning font-medium">
                已修改
              </span>
            )}
          </div>
          <div className="flex items-center gap-2">
            <button
              onClick={handleValidate}
              className="flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-xs font-medium bg-surface-light border border-border hover:bg-surface-lighter transition-colors"
            >
              <CheckCircle className="w-3.5 h-3.5" />
              校验
            </button>
            <button
              onClick={handleSave}
              disabled={saving || !isDirty}
              className={clsx(
                "flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-xs font-medium transition-all",
                isDirty
                  ? "bg-primary text-white hover:bg-primary-dark"
                  : "bg-surface-light text-text-dim border border-border cursor-not-allowed",
              )}
            >
              {saving ? (
                <Loader2 className="w-3.5 h-3.5 animate-spin" />
              ) : (
                <Save className="w-3.5 h-3.5" />
              )}
              保存
            </button>
            <button
              onClick={handleClose}
              className="p-1.5 rounded-lg hover:bg-surface-lighter text-text-dim hover:text-text transition"
            >
              <X className="w-4 h-4" />
            </button>
          </div>
        </div>

        {/* Status bar */}
        {(error || warnings.length > 0 || saveSuccess || validateSuccess) && (
          <div className="px-5 py-2 border-b border-border shrink-0 space-y-1">
            {error && (
              <div className="flex items-start gap-2 text-danger text-xs">
                <AlertTriangle className="w-3.5 h-3.5 shrink-0 mt-0.5" />
                <span>{error}</span>
              </div>
            )}
            {warnings.map((w, i) => (
              <div key={i} className="flex items-start gap-2 text-warning text-xs">
                <AlertTriangle className="w-3.5 h-3.5 shrink-0 mt-0.5" />
                <span>{w}</span>
              </div>
            ))}
            {validateSuccess && !error && warnings.length === 0 && (
              <div className="flex items-center gap-2 text-success text-xs">
                <CheckCircle className="w-3.5 h-3.5" />
                <span>格式校验通过 ✓</span>
              </div>
            )}
            {saveSuccess && !error && (
              <div className="flex items-center gap-2 text-success text-xs">
                <CheckCircle className="w-3.5 h-3.5" />
                <span>已保存成功{warnings.length > 0 ? "（有警告）" : ""}</span>
              </div>
            )}
          </div>
        )}

        {/* Body: Sidebar + Editor */}
        <div className="flex-1 flex overflow-hidden">
          {/* File Sidebar */}
          <div className="w-48 shrink-0 border-r border-border bg-surface-light overflow-y-auto">
            <div className="px-3 py-2 border-b border-border">
              <div className="flex items-center gap-1.5 text-[10px] font-semibold text-text-dim uppercase tracking-wider">
                <FolderOpen className="w-3 h-3" />
                ~/.ssh
              </div>
            </div>
            <div className="py-1">
              {configFiles.map((f) => {
                const isActive = f.path === activeFilePath;
                const tabEntry = tabs.get(f.path);
                const isTabDirty = tabEntry?.dirty ?? false;
                return (
                  <button
                    key={f.path}
                    onClick={() => selectFile(f)}
                    className={clsx(
                      "w-full flex items-center gap-2 px-3 py-1.5 text-left text-xs transition-colors",
                      isActive
                        ? "bg-primary/15 text-primary font-medium"
                        : "text-text-dim hover:bg-surface-lighter hover:text-text",
                    )}
                  >
                    <File className="w-3 h-3 shrink-0" />
                    <span className="truncate flex-1">{f.name}</span>
                    {isTabDirty && (
                      <span className="w-1.5 h-1.5 rounded-full bg-warning shrink-0" />
                    )}
                    {f.host_count > 0 && (
                      <span className="text-[10px] text-text-dim shrink-0">{f.host_count}</span>
                    )}
                  </button>
                );
              })}
              {configFiles.length <= 1 && (
                <div className="px-3 py-3 text-[10px] text-text-dim">
                  未发现 Include 引用的子配置
                </div>
              )}
            </div>
          </div>

          {/* Editor */}
          <div className="flex-1 overflow-hidden">
            {loading ? (
              <div className="flex items-center justify-center h-full">
                <Loader2 className="w-6 h-6 text-primary animate-spin" />
              </div>
            ) : !activeTab ? (
              <div className="flex items-center justify-center h-full text-text-dim text-sm">
                请从左侧选择一个配置文件
              </div>
            ) : (
              <div className="h-full flex">
                {/* Line numbers */}
                <div
                  ref={lineNumbersRef}
                  className="py-3 px-2 text-right text-text-dim text-xs font-mono select-none overflow-hidden border-r border-border bg-surface-light shrink-0 leading-[1.625rem]"
                >
                  {content.split("\n").map((_, i) => (
                    <div key={i}>{i + 1}</div>
                  ))}
                </div>
                {/* Textarea */}
                <textarea
                  ref={textareaRef}
                  value={content}
                  onChange={(e) => updateContent(e.target.value)}
                  spellCheck={false}
                  className="flex-1 h-full resize-none bg-transparent text-sm font-mono p-3 focus:outline-none leading-[1.625rem] overflow-auto"
                  style={{ tabSize: 4 }}
                />
              </div>
            )}
          </div>
        </div>

        {/* Footer */}
        <div className="flex items-center justify-between px-5 py-2 border-t border-border text-[10px] text-text-dim shrink-0">
          <span>
            {content.split("\n").length} 行 · {content.length} 字符
            {configFiles.length > 1 && ` · ${configFiles.length} 个配置文件`}
          </span>
          <span>⌘S 保存 · 保存时自动校验 · 自动备份</span>
        </div>
      </div>
    </div>
  );
}

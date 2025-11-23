/**
 * Notes - Per-Session Notes
 * For demo/training purposes - stored in memory only
 */

import { useState, useEffect } from 'react';
import { StickyNote, Save, Trash2, Plus, Clock, Tag, Pin, RefreshCw } from 'lucide-react';
import { clsx } from 'clsx';
import toast from 'react-hot-toast';
import { simulateSessionNotes } from '../../lib/tauri';
import type { SimulatedNote } from '../../types';

interface NotesProps {
  sessionId: string;
}

export function Notes({ sessionId }: NotesProps) {
  const [notes, setNotes] = useState<SimulatedNote[]>([]);
  const [selectedNote, setSelectedNote] = useState<SimulatedNote | null>(null);
  const [editTitle, setEditTitle] = useState('');
  const [editContent, setEditContent] = useState('');
  const [editTags, setEditTags] = useState('');
  const [isLoading, setIsLoading] = useState(false);

  const loadNotes = async () => {
    setIsLoading(true);
    try {
      const data = await simulateSessionNotes(sessionId);
      setNotes(data);
    } catch (error) {
      console.error('Failed to load notes:', error);
      toast.error('Failed to load notes');
    } finally {
      setIsLoading(false);
    }
  };

  useEffect(() => {
    loadNotes();
  }, [sessionId]);

  const handleNewNote = () => {
    const note: SimulatedNote = {
      id: `note-${Date.now()}`,
      title: 'Untitled Note',
      content: '',
      tags: [],
      created_at: new Date().toISOString(),
      updated_at: new Date().toISOString(),
      pinned: false,
      color: null,
    };
    setNotes(prev => [note, ...prev]);
    setSelectedNote(note);
    setEditTitle(note.title);
    setEditContent(note.content);
    setEditTags('');
  };

  const handleSelectNote = (note: SimulatedNote) => {
    // Save current note first
    if (selectedNote) {
      handleSaveNote();
    }
    setSelectedNote(note);
    setEditTitle(note.title);
    setEditContent(note.content);
    setEditTags(note.tags.join(', '));
  };

  const handleSaveNote = () => {
    if (!selectedNote) return;

    setNotes(prev => prev.map(n => {
      if (n.id !== selectedNote.id) return n;
      return {
        ...n,
        title: editTitle || 'Untitled Note',
        content: editContent,
        tags: editTags.split(',').map(t => t.trim()).filter(Boolean),
        updated_at: new Date().toISOString(),
      };
    }));
    toast.success('Note saved');
  };

  const handleDeleteNote = (id: string) => {
    setNotes(prev => prev.filter(n => n.id !== id));
    if (selectedNote?.id === id) {
      setSelectedNote(null);
      setEditTitle('');
      setEditContent('');
      setEditTags('');
    }
    toast.success('Note deleted');
  };

  const handleTogglePin = (id: string) => {
    setNotes(prev => prev.map(n => {
      if (n.id !== id) return n;
      return { ...n, pinned: !n.pinned };
    }));
  };

  // Sort notes: pinned first, then by date
  const sortedNotes = [...notes].sort((a, b) => {
    if (a.pinned !== b.pinned) return a.pinned ? -1 : 1;
    return new Date(b.updated_at).getTime() - new Date(a.updated_at).getTime();
  });

  return (
    <div className="h-full flex flex-col bg-dark-900">
      {/* Header */}
      <div className="p-4 border-b border-dark-600 bg-dark-800">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-2">
            <StickyNote className="text-pink-400" size={20} />
            <h2 className="text-lg font-semibold text-text-primary">Notes</h2>
            <span className="text-xs bg-pink-500/20 text-pink-400 px-2 py-0.5 rounded">SESSION</span>
            {isLoading && <RefreshCw size={12} className="text-pink-400 animate-spin ml-2" />}
          </div>
          <button
            onClick={handleNewNote}
            className="px-3 py-1.5 bg-pink-500 text-white rounded text-sm font-medium flex items-center gap-1.5 hover:bg-pink-600 transition-colors"
          >
            <Plus size={14} />
            New Note
          </button>
        </div>
        <p className="text-xs text-text-muted mt-1">Per-session notes (memory only)</p>
      </div>

      <div className="flex-1 flex overflow-hidden">
        {/* Notes List */}
        <div className="w-72 border-r border-dark-600 overflow-y-auto">
          {sortedNotes.length === 0 ? (
            <div className="p-4 text-center text-text-muted">
              <StickyNote size={32} className="mx-auto mb-2 opacity-30" />
              <p className="text-sm">No notes yet</p>
              <p className="text-xs mt-1">Click "New Note" to start</p>
            </div>
          ) : (
            <div className="divide-y divide-dark-600">
              {sortedNotes.map(note => (
                <button
                  key={note.id}
                  onClick={() => handleSelectNote(note)}
                  className={clsx(
                    'w-full p-3 text-left hover:bg-dark-700 transition-colors',
                    selectedNote?.id === note.id && 'bg-dark-700 border-l-2 border-l-pink-400'
                  )}
                >
                  <div className="flex items-center justify-between">
                    <div className="text-sm font-medium text-text-primary truncate flex-1">
                      {note.title}
                    </div>
                    {note.pinned && <Pin size={12} className="text-pink-400 ml-1" />}
                  </div>
                  <div className="text-xs text-text-muted mt-1 line-clamp-2">
                    {note.content || 'Empty note...'}
                  </div>
                  <div className="flex items-center gap-2 mt-2">
                    <Clock size={10} className="text-text-muted" />
                    <span className="text-[10px] text-text-muted">
                      {new Date(note.updated_at).toLocaleDateString()}
                    </span>
                    {note.tags.length > 0 && (
                      <>
                        <Tag size={10} className="text-text-muted ml-2" />
                        <span className="text-[10px] text-pink-400">
                          {note.tags.length}
                        </span>
                      </>
                    )}
                  </div>
                  {note.tags.length > 0 && (
                    <div className="flex flex-wrap gap-1 mt-2">
                      {note.tags.slice(0, 3).map(tag => (
                        <span key={tag} className="text-[10px] px-1.5 py-0.5 bg-pink-500/10 text-pink-400 rounded">
                          {tag}
                        </span>
                      ))}
                      {note.tags.length > 3 && (
                        <span className="text-[10px] text-text-muted">+{note.tags.length - 3}</span>
                      )}
                    </div>
                  )}
                </button>
              ))}
            </div>
          )}
        </div>

        {/* Note Editor */}
        <div className="flex-1 flex flex-col overflow-hidden">
          {selectedNote ? (
            <>
              {/* Editor Header */}
              <div className="p-3 border-b border-dark-600 flex items-center gap-3">
                <input
                  type="text"
                  value={editTitle}
                  onChange={e => setEditTitle(e.target.value)}
                  placeholder="Note title..."
                  className="flex-1 px-3 py-1.5 bg-dark-700 border border-dark-600 rounded text-sm text-text-primary font-medium focus:border-pink-400/50 focus:outline-none"
                />
                <button
                  onClick={() => handleTogglePin(selectedNote.id)}
                  className={clsx(
                    'p-1.5 rounded transition-colors',
                    selectedNote.pinned
                      ? 'bg-pink-500/20 text-pink-400'
                      : 'bg-dark-700 text-text-secondary hover:text-text-primary'
                  )}
                  title={selectedNote.pinned ? 'Unpin' : 'Pin'}
                >
                  <Pin size={14} />
                </button>
                <button
                  onClick={handleSaveNote}
                  className="px-3 py-1.5 bg-pink-500/20 text-pink-400 rounded text-xs font-medium flex items-center gap-1.5 hover:bg-pink-500/30 transition-colors"
                >
                  <Save size={12} />
                  Save
                </button>
                <button
                  onClick={() => handleDeleteNote(selectedNote.id)}
                  className="px-3 py-1.5 bg-dark-700 text-text-secondary rounded text-xs font-medium flex items-center gap-1.5 hover:text-red-400 transition-colors"
                >
                  <Trash2 size={12} />
                </button>
              </div>

              {/* Tags Input */}
              <div className="px-3 py-2 border-b border-dark-600">
                <div className="flex items-center gap-2">
                  <Tag size={12} className="text-text-muted" />
                  <input
                    type="text"
                    value={editTags}
                    onChange={e => setEditTags(e.target.value)}
                    placeholder="Tags (comma separated)..."
                    className="flex-1 bg-transparent text-xs text-text-primary placeholder:text-text-muted focus:outline-none"
                  />
                </div>
              </div>

              {/* Content Editor */}
              <div className="flex-1 p-3">
                <textarea
                  value={editContent}
                  onChange={e => setEditContent(e.target.value)}
                  placeholder="Write your notes here...

Useful for:
• Target information
• Discovered credentials
• Attack paths
• Observations
• Next steps"
                  className="w-full h-full bg-dark-800 border border-dark-600 rounded p-3 text-sm text-text-primary placeholder:text-text-muted resize-none focus:border-pink-400/50 focus:outline-none font-mono"
                />
              </div>
            </>
          ) : (
            <div className="h-full flex items-center justify-center text-text-muted">
              <div className="text-center">
                <StickyNote size={48} className="mx-auto mb-4 opacity-20" />
                <p>Select a note to edit</p>
                <p className="text-xs mt-1">or create a new one</p>
              </div>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}

export default Notes;

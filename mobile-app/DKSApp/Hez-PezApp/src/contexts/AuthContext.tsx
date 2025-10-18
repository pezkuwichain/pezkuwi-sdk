import React, { createContext, useContext, useState, useEffect } from 'react';
import { supabase } from '@/lib/supabase';
import { User, Session } from '@supabase/supabase-js';

interface Profile {
  id: string;
  username?: string;
  full_name?: string;
  avatar_url?: string;
  bio?: string;
  wallet_address?: string;
  role: 'member' | 'delegate' | 'moderator' | 'admin';
  reputation_score: number;
  created_at: string;
  updated_at: string;
}

interface AuthContextType {
  user: User | null;
  session: Session | null;
  profile: Profile | null;
  loading: boolean;
  signUp: (email: string, password: string, metadata?: any) => Promise<any>;
  signIn: (email: string, password: string) => Promise<any>;
  signOut: () => Promise<void>;
  updateProfile: (updates: Partial<Profile>) => Promise<any>;
  checkPermission: (permission: string) => boolean;
}

const AuthContext = createContext<AuthContextType | undefined>(undefined);

export const useAuth = () => {
  const context = useContext(AuthContext);
  if (!context) throw new Error('useAuth must be used within AuthProvider');
  return context;
};

export const AuthProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const [user, setUser] = useState<User | null>(null);
  const [session, setSession] = useState<Session | null>(null);
  const [profile, setProfile] = useState<Profile | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    supabase.auth.getSession().then(({ data: { session } }) => {
      setSession(session);
      setUser(session?.user ?? null);
      if (session?.user) {
        fetchProfile(session.user.id);
      }
      setLoading(false);
    });

    const { data: { subscription } } = supabase.auth.onAuthStateChange((_event, session) => {
      setSession(session);
      setUser(session?.user ?? null);
      if (session?.user) {
        fetchProfile(session.user.id);
      } else {
        setProfile(null);
      }
    });

    return () => subscription.unsubscribe();
  }, []);

  const fetchProfile = async (userId: string) => {
    const { data, error } = await supabase
      .from('profiles')
      .select('*')
      .eq('id', userId)
      .single();

    if (!error && data) {
      setProfile(data);
    }
  };

  const signUp = async (email: string, password: string, metadata?: any) => {
    const { data, error } = await supabase.auth.signUp({
      email,
      password,
      options: { data: metadata }
    });

    if (!error && data.user) {
      await supabase.functions.invoke('auth-profile', {
        body: {
          action: 'create-profile',
          userId: data.user.id,
          profileData: { ...metadata, email }
        }
      });
    }

    return { data, error };
  };

  const signIn = async (email: string, password: string) => {
    const result = await supabase.auth.signInWithPassword({ email, password });
    
    // Log the login activity
    if (!result.error && result.data.user) {
      await supabase.functions.invoke('log-activity', {
        body: {
          user_id: result.data.user.id,
          action: 'login',
          details: { method: 'email_password' }
        }
      });
    }
    
    return result;
  };

  const signOut = async () => {
    // Log the logout activity before signing out
    if (user) {
      await supabase.functions.invoke('log-activity', {
        body: {
          user_id: user.id,
          action: 'logout',
          details: {}
        }
      });
    }
    
    await supabase.auth.signOut();
  };

  const updateProfile = async (updates: Partial<Profile>) => {
    if (!user) return { error: 'No user logged in' };

    const { data, error } = await supabase
      .from('profiles')
      .update(updates)
      .eq('id', user.id)
      .select()
      .single();

    if (!error && data) {
      setProfile(data);
    }

    return { data, error };
  };

  const checkPermission = (permission: string): boolean => {
    if (!profile) return false;

    const rolePermissions: Record<string, string[]> = {
      admin: ['all'],
      moderator: ['moderate', 'vote', 'create_proposal', 'delegate'],
      delegate: ['vote', 'create_proposal', 'delegate'],
      member: ['vote']
    };

    return rolePermissions[profile.role]?.includes('all') || 
           rolePermissions[profile.role]?.includes(permission) || false;
  };

  return (
    <AuthContext.Provider value={{
      user,
      session,
      profile,
      loading,
      signUp,
      signIn,
      signOut,
      updateProfile,
      checkPermission
    }}>
      {children}
    </AuthContext.Provider>
  );
};
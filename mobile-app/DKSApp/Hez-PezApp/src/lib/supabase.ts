import { createClient } from '@supabase/supabase-js';


// Initialize Supabase client
// Using direct values from project configuration
const supabaseUrl = 'https://vbhftvdayqfmcgmzdxfv.supabase.co';
const supabaseKey = 'sb_publishable_Aq6cShprdtxYyUsohmsquQ_OurU7w07';
const supabase = createClient(supabaseUrl, supabaseKey);


export { supabase };
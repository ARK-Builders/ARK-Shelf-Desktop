import {
  ListItem,
  Grid,
  Button,
  ListItemText,
  Typography,
} from '@mui/material';
import { clipboard, invoke } from '@tauri-apps/api';
import { open } from '@tauri-apps/api/shell';
import useSWR from 'swr';

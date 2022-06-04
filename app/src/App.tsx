import { open } from '@tauri-apps/api/shell';
import {
  AppBar,
  Button,
  Card,
  CardContent,
  Grid,
  List,
  ListItem,
  ListItemText,
  Toolbar,
  Container,
  Typography,
  TextField,
} from '@mui/material';
import { useState, useEffect } from 'react';
import { dialog, invoke, clipboard } from '@tauri-apps/api';
import { useForm, SubmitHandler } from 'react-hook-form';

interface LinkInfo {
  title: string;
  desc: string;
  url: string;
}
const Home = () => {
  const [, setLinkNames] = useState<string[]>([]);
  const [linkInfos, setLinkInfos] = useState<LinkInfo[]>([]);
  // const [refresh, setRefresh] = useState(false);
  const { register, handleSubmit } = useForm<LinkInfo>();
  const createLink: SubmitHandler<LinkInfo> = (data) => {
    invoke('create_link', {
      ...data,
    }).then(() => {
      dialog.message('Link Created!');
      refreshInfo();
      // setRefresh(true)
    });
  };

  const refreshInfo = async () => {
    const names = (await invoke('read_link_list')) as string[];
    setLinkNames(names);
    const links = await Promise.all(
      names.map(async (val) => {
        const link = (await invoke('read_link', { name: val })) as LinkInfo;
        return link;
      })
    );
    setLinkInfos(links);
  };

  useEffect(() => {
    refreshInfo();
  }, []);

  return (
    <>
      <AppBar position='fixed'>
        <Toolbar>
          <Typography
            variant='h6'
            sx={{
              flexGrow: 1,
            }}>
            ARK Shelf
          </Typography>
        </Toolbar>
      </AppBar>
      <Toolbar />
      <Container
        sx={{
          mt: 2,
        }}>
        <Grid container spacing={8}>
          <Grid item xs={8}>
            <Card>
              <CardContent>
                <List>
                  {linkInfos.map((val) => {
                    return (
                      <ListItem
                        dense
                        key={val.title}
                        secondaryAction={
                          <Grid container>
                            <Grid item m='auto'>
                              <Button
                                onClick={() => {
                                  clipboard.writeText(val.url);
                                }}>
                                {'COPY'}
                              </Button>
                            </Grid>
                            <Grid item m='auto' p='auto'>
                              <Button
                                onClick={() => {
                                  open(val.url.toString());
                                }}>
                                OPEN
                              </Button>
                            </Grid>
                          </Grid>
                        }>
                        <ListItemText
                          primary={
                            <Typography variant='h6'>{val.title}</Typography>
                          }
                          secondary={
                            <Typography variant='subtitle2'>
                              {val.desc}
                            </Typography>
                          }></ListItemText>
                      </ListItem>
                    );
                  })}
                </List>
              </CardContent>
            </Card>
          </Grid>
          <Grid item xs={4}>
            <form onSubmit={handleSubmit(createLink)}>
              <TextField
                fullWidth
                label='url'
                margin='normal'
                {...register('url', { required: true })}></TextField>
              <TextField
                fullWidth
                label='title'
                margin='normal'
                {...register('title', { required: true })}></TextField>
              <TextField
                fullWidth
                label='description'
                margin='normal'
                {...register('desc', { required: true })}></TextField>

              <Button type='submit'>Create</Button>
            </form>
          </Grid>
        </Grid>
      </Container>
    </>
  );
};
export default Home;

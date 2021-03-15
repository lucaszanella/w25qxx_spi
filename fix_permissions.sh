#docker writes to root folder so we generally cannot edit or execute git push, so we fix things with this script
#sudo chown -R $USER: engine/
sudo chmod -R u=rwX,g=rX,o=rX .[^.]*
sudo chown -R $USER:$USER .[^.]*
sudo chown -R $USER:$USER *

